//! 🧪 Runs the staged sender and queue against a turn peer and the exact PDF fixture.
extern crate self as semio_framework;
extern crate self as semio_framework_diagnostic;

pub struct Fault(&'static str);
pub enum FaultOrigin { Framework }
pub struct FaultCode;
impl FaultCode { fn new(_: &str) -> Self { Self } }
impl Fault {
    fn new(_: FaultOrigin, _: FaultCode, message: &'static str) -> Self { Self(message) }
    fn describe(&self) -> String { self.0.to_owned() }
}

#[allow(dead_code)]
mod kernel {
    pub const COMMAND_MAXIMUM_BYTES: usize = 8 * 1024 * 1024;
    include!(env!("SEMIO_NATIVE_SOURCE_WIRE"));
    pub enum Event {
        NaturalSourcePrepare(NaturalSourcePrepare), NaturalSourceControl(NaturalSourceControlEvent),
        CommandIngressPage { cursor: CommandPageCursor, bytes: FixedCommandPage },
        OriginalMediaControl(OriginalMediaControlEvent), Wake,
    }
}

mod sender { include!(env!("SEMIO_NATIVE_SOURCE_PUMP")); }
mod queue { include!(env!("SEMIO_NATIVE_SOURCE_QUEUE")); }
use kernel::{NaturalSourceWireControl as Control, NaturalSourceWirePhase as Phase, NaturalSourceWireResult as Receipt, *};
use std::{cell::Cell, future::{Future, ready}, fs::File, io::{self, Read, Write}, task::{Context, Poll, Waker}};

const PAGE_BYTES: usize = parse_dec(env!("SEMIO_NATIVE_SOURCE_PAGE_BYTES")) as usize;
const PAGE_COUNT: u32 = parse_dec(env!("SEMIO_NATIVE_SOURCE_PAGE_COUNT")) as u32;
const TAIL_BYTES: usize = parse_dec(env!("SEMIO_NATIVE_SOURCE_TAIL_BYTES")) as usize;
const INSTANCE: u32 = parse_dec(env!("SEMIO_NATIVE_SOURCE_INSTANCE")) as u32;
const EXTENT: u32 = parse_dec(env!("SEMIO_NATIVE_SOURCE_EXTENT")) as u32;

const fn parse_dec(input: &str) -> u64 {
    let mut value = 0;
    let mut i = 0;
    while i < input.len() {
        value = value * 10 + (input.as_bytes()[i] - b'0') as u64;
        i += 1;
    }
    value
}

fn sequence() -> u64 { u64::from_str_radix(env!("SEMIO_NATIVE_SOURCE_SEQUENCE"), 16).unwrap() }
fn generation() -> u64 { u64::from_str_radix(env!("SEMIO_NATIVE_SOURCE_GENERATION"), 16).unwrap() }

fn prepare() -> NaturalSourcePrepare {
    NaturalSourcePrepare {
        instance_id: INSTANCE, seq: sequence(), extent: EXTENT,
        declaration: NaturalSourceWireDeclaration {
            format_kind: "s.stdio.pdf@1.7".into(), extension: ".pdf".into(), media_type: "application/pdf".into(), binary: true,
        },
    }
}

fn loan() -> NaturalSourceWireLoan {
    let prepare = prepare();
    NaturalSourceWireLoan {
        operation_id: env!("SEMIO_NATIVE_SOURCE_OPERATION").into(),
        generation: env!("SEMIO_NATIVE_SOURCE_GENERATION").into(),
        instance_id: prepare.instance_id, declaration: prepare.declaration, extent: prepare.extent,
    }
}

fn ack(phase: Phase, page_index: Option<u32>, result: Receipt) -> CommandIngressStatus {
    let loan = (phase != Phase::Prepare || !matches!(result, Receipt::Refused | Receipt::Stale)).then(loan);
    CommandIngressStatus::NaturalSourceAck(NaturalSourceWireAck {
        loan, owner: u64::from(INSTANCE), seq: sequence(), kind: NATURAL_SOURCE_COMMAND_KIND,
        extent: EXTENT, phase, page_index, result,
    })
}

struct MeasuredReader<'a, R> {
    input: R,
    reads: &'a Cell<usize>,
    maximum: &'a Cell<usize>,
}

impl<R: Read> Read for MeasuredReader<'_, R> {
    fn read(&mut self, output: &mut [u8]) -> io::Result<usize> {
        self.maximum.set(self.maximum.get().max(output.len()));
        let count = self.input.read(output)?;
        self.reads.set(self.reads.get() + count);
        Ok(count)
    }
}

fn complete<T>(future: impl Future<Output = T>) -> T {
    let mut future = std::pin::pin!(future);
    let mut context = Context::from_waker(Waker::noop());
    loop {
        if let Poll::Ready(output) = future.as_mut().poll(&mut context) { return output; }
    }
}

fn queue(event: Event) -> Event {
    let admitted = queue::QueuedOriginalMedia::admit(event).ok().expect("native queue admits the exact crossing");
    match &admitted {
        queue::QueuedOriginalMedia::Page(..) => assert_eq!(admitted.credits(), (1, PAGE_BYTES)),
        queue::QueuedOriginalMedia::Poll => assert_eq!(admitted.credits(), (0, 0)),
        _ => {
            let (pages, bytes) = admitted.credits();
            assert_eq!(pages, 0);
            assert!((1..=PAGE_BYTES).contains(&bytes));
        }
    }
    admitted.into_event()
}

fn check_control(control: &NaturalSourceControlEvent) {
    assert_eq!(control.loan, loan());
    assert_eq!(control.owner, u64::from(INSTANCE));
    assert_eq!(control.seq, sequence());
    assert_eq!(control.kind, NATURAL_SOURCE_COMMAND_KIND);
    assert_eq!(control.extent, EXTENT);
}

fn fixture() -> File { File::open(std::env::var_os("SEMIO_NATIVE_SOURCE_FIXTURE").unwrap()).unwrap() }

#[test]
fn real_pdf_through_large_native_ingress_with_backpressure_and_pending_completion() {
    let reads = Cell::new(0);
    let maximum = Cell::new(0);
    let progress = Cell::new(0);
    let mut output = File::create(std::env::var_os("SEMIO_NATIVE_SOURCE_CAPTURE").unwrap()).unwrap();
    let mut offered_page = 0;
    let mut first_page_retries = 0;
    let mut pending = None;
    let mut final_idle = false;
    let mut finished = false;
    let result = complete(sender::stream(
        prepare(), MeasuredReader { input: fixture(), reads: &reads, maximum: &maximum },
        |event, closing| {
            assert!(!closing);
            let status = match queue(event) {
                Event::NaturalSourcePrepare(selected) => {
                    assert_eq!(selected, prepare());
                    assert_eq!(reads.get(), 0);
                    pending = Some((Phase::Prepare, None));
                    ack(Phase::Prepare, None, Receipt::Pending)
                }
                Event::NaturalSourceControl(control) => {
                    check_control(&control);
                    match control.control {
                        Control::Begin => {
                            assert_eq!(reads.get(), 0);
                            ack(Phase::Begin, None, Receipt::Accepted)
                        }
                        Control::Complete => {
                            assert_eq!(offered_page, PAGE_COUNT);
                            assert_eq!(progress.get(), u64::from(EXTENT));
                            assert_eq!(reads.get(), EXTENT as usize);
                            pending = Some((Phase::Complete, None));
                            ack(Phase::Complete, None, Receipt::Pending)
                        }
                        Control::Cancel => panic!("successful open cannot cancel"),
                    }
                }
                Event::CommandIngressPage { cursor, bytes } => {
                    let index = cursor.page_index;
                    assert_eq!(index, offered_page);
                    assert_eq!(cursor.owner, u64::from(INSTANCE));
                    assert_eq!(cursor.instance, INSTANCE);
                    assert_eq!(cursor.generation, generation());
                    assert_eq!(cursor.seq, sequence());
                    assert_eq!(cursor.kind, NATURAL_SOURCE_COMMAND_KIND);
                    assert_eq!(cursor.metadata, EXTENT);
                    assert_eq!(cursor.page_count, PAGE_COUNT);
                    assert_eq!(cursor.command_count, 1);
                    assert_eq!(cursor.command_index, 0);
                    assert_eq!(cursor.item_count, 1);
                    let count = if index == PAGE_COUNT - 1 { TAIL_BYTES } else { PAGE_BYTES };
                    assert_eq!(bytes.len(), count);
                    assert_eq!(reads.get(), index as usize * PAGE_BYTES + count);
                    assert_eq!(progress.get(), u64::from(index) * PAGE_BYTES as u64);
                    if index == 0 && first_page_retries < 2 {
                        first_page_retries += 1;
                        if first_page_retries == 1 { CommandIngressStatus::Backpressure(cursor) }
                        else { ack(Phase::Page, Some(index), Receipt::Retry) }
                    } else {
                        output.write_all(bytes.as_slice()).unwrap();
                        offered_page += 1;
                        pending = Some((Phase::Page, Some(index)));
                        ack(Phase::Page, Some(index), Receipt::Pending)
                    }
                }
                Event::Wake => {
                    let (phase, index) = pending.take().expect("Wake follows pending even after a full page");
                    if phase == Phase::Complete && !final_idle {
                        final_idle = true;
                        pending = Some((phase, index));
                        CommandIngressStatus::Idle
                    } else {
                        if phase == Phase::Complete { finished = true; }
                        ack(phase, index, Receipt::Accepted)
                    }
                }
                _ => panic!("unexpected crossing"),
            };
            ready(Ok(status))
        },
        || true,
        |done, extent| {
            assert!(done > progress.get());
            assert_eq!(extent, u64::from(EXTENT));
            progress.set(done);
        },
    ));
    let answer = result.unwrap().unwrap();
    assert_eq!(answer.loan, Some(loan()));
    assert_eq!(answer.phase, Phase::Complete);
    assert_eq!(answer.seq, sequence());
    assert_eq!(answer.result, Receipt::Accepted);
    assert!(finished);
    assert_eq!(maximum.get(), PAGE_BYTES);
    assert_eq!(first_page_retries, 2);
}

#[test]
fn cancellation_during_pending_page_closes_and_never_reads_another_page() {
    let reads = Cell::new(0);
    let maximum = Cell::new(0);
    let current = Cell::new(true);
    let mut closed = false;
    let mut close_pending = false;
    let result = complete(sender::stream(
        prepare(), MeasuredReader { input: fixture(), reads: &reads, maximum: &maximum },
        |event, closing| {
            let status = match queue(event) {
                Event::NaturalSourcePrepare(_) => ack(Phase::Prepare, None, Receipt::Accepted),
                Event::NaturalSourceControl(control) => match control.control {
                    Control::Begin => ack(Phase::Begin, None, Receipt::Accepted),
                    Control::Cancel => {
                        assert!(closing);
                        check_control(&control);
                        close_pending = true;
                        ack(Phase::Cancel, None, Receipt::Pending)
                    }
                    Control::Complete => panic!("cancelled source cannot complete"),
                },
                Event::CommandIngressPage { cursor, .. } => {
                    assert_eq!(cursor.page_index, 0);
                    current.set(false);
                    ack(Phase::Page, Some(0), Receipt::Pending)
                }
                Event::Wake => {
                    assert!(closing && close_pending);
                    closed = true;
                    ack(Phase::Cancel, None, Receipt::Accepted)
                }
                _ => panic!("unexpected crossing"),
            };
            ready(Ok(status))
        },
        || current.get(), |_, _| panic!("pending page never advanced"),
    ));
    assert_eq!(result, Ok(None));
    assert!(closed);
    assert_eq!(reads.get(), PAGE_BYTES);
}

struct CancelOnRead<'a, R> {
    input: R,
    reads: &'a Cell<usize>,
    current: &'a Cell<bool>,
    at: usize,
}

impl<R: Read> Read for CancelOnRead<'_, R> {
    fn read(&mut self, output: &mut [u8]) -> io::Result<usize> {
        let count = self.input.read(output)?;
        self.reads.set(self.reads.get() + count);
        if self.reads.get() >= self.at { self.current.set(false); }
        Ok(count)
    }
}

fn accepting(event: Event, closing: bool, pages: &Cell<u32>, closed: &Cell<bool>) -> CommandIngressStatus {
    match queue(event) {
        Event::NaturalSourcePrepare(_) => ack(Phase::Prepare, None, Receipt::Accepted),
        Event::NaturalSourceControl(control) => match control.control {
            Control::Begin => ack(Phase::Begin, None, Receipt::Accepted),
            Control::Cancel => { assert!(closing); check_control(&control); closed.set(true); ack(Phase::Cancel, None, Receipt::Accepted) }
            Control::Complete => panic!("cancelled source cannot complete"),
        },
        Event::CommandIngressPage { cursor, .. } => {
            assert_eq!(cursor.page_index, pages.get());
            pages.set(pages.get() + 1);
            ack(Phase::Page, Some(cursor.page_index), Receipt::Accepted)
        }
        _ => panic!("unexpected crossing"),
    }
}

#[test]
fn cancellation_deep_in_the_large_source_stops_after_the_acknowledged_page() {
    let reads = Cell::new(0);
    let maximum = Cell::new(0);
    let current = Cell::new(true);
    let pages = Cell::new(0);
    let closed = Cell::new(false);
    let cut = PAGE_COUNT as usize / 2;
    let result = complete(sender::stream(
        prepare(), MeasuredReader { input: fixture(), reads: &reads, maximum: &maximum },
        |event, closing| ready(Ok(accepting(event, closing, &pages, &closed))),
        || current.get(), |offset, _| if offset >= cut as u64 * PAGE_BYTES as u64 { current.set(false) },
    ));
    assert_eq!(result, Ok(None));
    assert!(closed.get());
    assert_eq!(pages.get() as usize, cut);
    assert_eq!(reads.get(), cut * PAGE_BYTES);
    assert!(maximum.get() <= PAGE_BYTES);
}

#[test]
fn cancellation_while_a_blocking_read_is_in_flight_drops_that_page_and_closes() {
    let reads = Cell::new(0);
    let current = Cell::new(true);
    let pages = Cell::new(0);
    let closed = Cell::new(false);
    let cut = (PAGE_COUNT as usize / 3) * PAGE_BYTES;
    let result = complete(sender::stream(
        prepare(), CancelOnRead { input: fixture(), reads: &reads, current: &current, at: cut },
        |event, closing| ready(Ok(accepting(event, closing, &pages, &closed))),
        || current.get(), |_, _| {},
    ));
    assert_eq!(result, Ok(None));
    assert!(closed.get());
    assert_eq!(reads.get(), cut);
    assert_eq!(pages.get() as usize, PAGE_COUNT as usize / 3 - 1);
}

#[test]
fn foreign_page_ack_cancels_the_exact_loan() {
    let mut closed = false;
    let result = complete(sender::stream(
        prepare(), fixture(),
        |event, closing| ready(Ok(match queue(event) {
            Event::NaturalSourcePrepare(_) => ack(Phase::Prepare, None, Receipt::Accepted),
            Event::NaturalSourceControl(control) if control.control == Control::Begin =>
                ack(Phase::Begin, None, Receipt::Accepted),
            Event::CommandIngressPage { cursor, .. } => {
                assert_eq!(cursor.page_index, 0);
                ack(Phase::Page, Some(1), Receipt::Accepted)
            }
            Event::NaturalSourceControl(control) => {
                assert_eq!(control.control, Control::Cancel);
                assert!(closing);
                check_control(&control);
                closed = true;
                ack(Phase::Cancel, None, Receipt::Accepted)
            }
            _ => panic!("unexpected crossing"),
        })),
        || true, |_, _| panic!("foreign page never advanced"),
    ));
    assert_eq!(result, Err("natural-source.foreign-ack".into()));
    assert!(closed);
}

#[test]
fn refused_prepare_does_not_read_source() {
    let reads = Cell::new(0);
    let maximum = Cell::new(0);
    let result = complete(sender::stream(
        prepare(), MeasuredReader { input: fixture(), reads: &reads, maximum: &maximum },
        |event, _| {
            assert!(matches!(queue(event), Event::NaturalSourcePrepare(_)));
            ready(Ok(ack(Phase::Prepare, None, Receipt::Refused)))
        },
        || true, |_, _| panic!("refused prepare never advances"),
    ));
    assert_eq!(result, Err("natural-source.refused".into()));
    assert_eq!(reads.get(), 0);
}

#[test]
fn short_and_grown_files_cancel_without_completion() {
    for input in [Box::new(fixture().take(u64::from(EXTENT) - 1)) as Box<dyn Read>,
        Box::new(fixture().chain(&b"!"[..]))]
    {
        let mut cancelled = false;
        let result = complete(sender::stream(
            prepare(), input, |event, closing| {
                ready(Ok(match queue(event) {
                    Event::NaturalSourcePrepare(_) => ack(Phase::Prepare, None, Receipt::Accepted),
                    Event::NaturalSourceControl(control) => match control.control {
                        Control::Begin => ack(Phase::Begin, None, Receipt::Accepted),
                        Control::Cancel => {
                            assert!(closing);
                            cancelled = true;
                            ack(Phase::Cancel, None, Receipt::Stale)
                        }
                        Control::Complete => panic!("changed source cannot complete"),
                    },
                    Event::CommandIngressPage { cursor, .. } => ack(Phase::Page, Some(cursor.page_index), Receipt::Accepted),
                    _ => panic!("unexpected crossing"),
                }))
            },
            || true, |_, _| {},
        ));
        assert!(result.is_err());
        assert!(cancelled);
    }
}

#[test]
fn png_keeps_its_owner_and_extent_policy() {
    let mut cursor = CommandPageCursor {
        owner: sequence(), seq: sequence(), instance: INSTANCE, generation: generation(),
        kind: ORIGINAL_MEDIA_COMMAND_KIND, command_index: 0, command_count: 1, item_count: 1,
        page_index: 0, page_count: 1, metadata: PAGE_BYTES as u32,
    };
    let event = |cursor: &CommandPageCursor| Event::CommandIngressPage {
        cursor: cursor.clone(), bytes: FixedCommandPage::try_copy_from(&[0; PAGE_BYTES]).ok().unwrap(),
    };
    assert!(queue::QueuedOriginalMedia::admit(event(&cursor)).is_ok());
    cursor.metadata = EXTENT;
    cursor.page_count = PAGE_COUNT;
    assert!(queue::QueuedOriginalMedia::admit(event(&cursor)).is_err());
    cursor.kind = NATURAL_SOURCE_COMMAND_KIND;
    assert!(queue::QueuedOriginalMedia::admit(event(&cursor)).is_err());
    cursor.owner = u64::from(INSTANCE);
    assert!(queue::QueuedOriginalMedia::admit(event(&cursor)).is_ok());
    cursor.generation = 0;
    assert!(queue::QueuedOriginalMedia::admit(event(&cursor)).is_err());
}
