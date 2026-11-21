//! 🧪️ Runs the native PNG sample-plane proof against a discontiguous retained source.

#[path = "paged-png-planes.rs"]
mod png;
#[path = "paged-png-paid-planes.rs"]
mod paid;
#[path = "paged-png-paid-probe.rs"]
mod paid_probe;

use std::{sync::atomic::{AtomicUsize, Ordering}, fs::File, io::{Read, Write}, path::PathBuf};
use png::{PngIdatProof, PngPlaneTarget, PngRefusal};
use semio_framework_value::{RetainedCloneGrant, paged::{PagedBytes, ByteReadSource}};

const INPUT_CAP: usize = 16 * 1024 * 1024;
const OUTPUT_CAP: usize = 4 * 1024 * 1024;
const PAGE: usize = 4096;
const GRANT: RetainedCloneGrant = RetainedCloneGrant {
    maximum_items: 1, maximum_copy_bytes: PAGE, maximum_capacity_bytes: 8192,
    maximum_release_bytes: 0, maximum_depth: 64,
};

struct MeasuredInput {
    data: PagedBytes<INPUT_CAP>,
    accessed: AtomicUsize,
}

impl ByteReadSource for MeasuredInput {
    fn byte_len(&self) -> usize { self.data.len() }
    fn byte_at(&self, at: usize) -> Option<u8> {
        self.accessed.fetch_add(1, Ordering::Relaxed);
        self.data.byte_at(at)
    }
    fn read_bytes(&self, at: usize, output: &mut [u8]) -> usize {
        self.accessed.fetch_add(output.len(), Ordering::Relaxed);
        self.data.read_bytes(at, output)
    }
}

fn push<const MAX: usize>(page: &mut PagedBytes<MAX>, value: u8) -> Result<(), PngRefusal> {
    for attempt in 0..16 {
        if page.retained_bytes().has_reserved_slot() { break; }
        assert!(attempt < 15, "PNG page reserve did not settle");
        let reserved = page.reserve_byte_backing(GRANT).map_err(|error| {
            eprintln!("[DEBUG] PNG plane reserve at {} bytes refused: {error:?}", page.len());
            PngRefusal::SampleTarget
        })?;
        assert!(reserved.retained_capacity_bytes <= 8192);
    }
    page.push_reserved_byte(value).map_err(|error| {
        eprintln!("[DEBUG] PNG plane push at {} bytes refused: {error:?}", page.len());
        PngRefusal::SampleTarget
    })
}

struct RasterPlanes {
    channels: usize,
    colors: PagedBytes<OUTPUT_CAP>,
    opacity: PagedBytes<OUTPUT_CAP>,
}

impl PngPlaneTarget for RasterPlanes {
    fn previous(&self, pixel: usize, channel: usize) -> Option<u8> {
        let value = if channel == self.channels { self.opacity.retained_bytes().get(pixel).copied() }
        else { self.colors.retained_bytes().get(pixel * self.channels + channel).copied() };
        if value.is_none() { eprintln!("[DEBUG] PNG predictor missing pixel {pixel} channel {channel}, color {}, alpha {}", self.colors.len(), self.opacity.len()); }
        value
    }

    fn put(&mut self, pixel: usize, channel: usize, value: u8) -> Result<(), PngRefusal> {
        if channel == self.channels {
            assert_eq!(self.opacity.len(), pixel);
            push(&mut self.opacity, value)
        } else {
            assert_eq!(self.colors.len(), pixel * self.channels + channel);
            push(&mut self.colors, value)
        }
    }
}

fn write_plane(output: PathBuf, source: &dyn ByteReadSource) {
    let mut output = std::io::BufWriter::new(File::create(output).unwrap());
    let mut block = [0; PAGE];
    let mut at = 0;
    while at < source.byte_len() {
        let copied = source.read_bytes(at, &mut block);
        assert!(copied > 0);
        output.write_all(&block[..copied]).unwrap();
        at += copied;
    }
}

fn run(input: PathBuf, output: PathBuf, cancel: Option<usize>, paid_mode: bool) -> Result<(), PngRefusal> {
    let mut file = File::open(input).unwrap();
    let mut raw: PagedBytes<INPUT_CAP> = PagedBytes::default();
    let mut block = [0; PAGE];
    loop {
        let count = file.read(&mut block).unwrap();
        if count == 0 { break; }
        for byte in &block[..count] { push(&mut raw, *byte)?; }
    }
    let source = MeasuredInput { data: raw, accessed: AtomicUsize::new(0) };
    let mut proof = PngIdatProof::new(source.byte_len(), 1024 * 1024, 10_000);
    let mut peak_reads = 0;
    let mut turns = 0;
    loop {
        assert!(proof.samples().is_none());
        source.accessed.store(0, Ordering::Relaxed);
        let done = proof.step(&source, PAGE)?;
        peak_reads = peak_reads.max(source.accessed.load(Ordering::Relaxed));
        turns += 1;
        if done { break; }
    }
    let mut samples = proof.samples().unwrap();
    if paid_mode { paid_probe::decode_paid(samples, &source, output, cancel); return Ok(()); }
    let extent = samples.extent();
    let mut planes = RasterPlanes {
        channels: usize::from(extent.color_samples), colors: PagedBytes::default(), opacity: PagedBytes::default(),
    };
    let mut decoded = 0;
    let mut result = Ok(false);
    while result == Ok(false) && cancel != Some(decoded) && decoded < 20_000_000 {
        source.accessed.store(0, Ordering::Relaxed);
        result = samples.step(&source, &mut planes, 32768);
        peak_reads = peak_reads.max(source.accessed.load(Ordering::Relaxed));
        decoded += 1;
    }
    assert!(peak_reads <= PAGE);
    assert!((0..256).any(|_| samples.close_history_step(1, 32768)));
    assert!(decoded < 20_000_000, "PNG decode stalled before publication");
    if !result? {
        println!("[DEBUG] Cancelled private PNG decode at {decoded} turns");
        return Ok(());
    }
    let pixels = extent.width as usize * extent.height as usize;
    assert_eq!(planes.colors.len(), pixels * planes.channels);
    assert_eq!(planes.opacity.len(), if extent.alpha { pixels } else { 0 });
    write_plane(output.with_extension("color"), &planes.colors);
    write_plane(output.with_extension("alpha"), &planes.opacity);
    println!("[DEBUG] {}x{} {} color samples, {} alpha, input {}, proof {} turns, decode {} turns, peak {} read bytes",
        extent.width, extent.height, planes.colors.len(), planes.opacity.len(), source.byte_len(), turns, decoded, peak_reads);
    Ok(())
}

fn main() {
    let mut args = std::env::args().skip(1);
    let input = PathBuf::from(args.next().unwrap());
    let output = PathBuf::from(args.next().unwrap());
    let mode = args.next();
    let paid = mode.as_deref() == Some("paid");
    let cancel = if paid { args.next() } else { mode };
    let result = run(input, output, cancel.map(|value| value.parse().unwrap()), paid);
    if let Err(refused) = result {
        eprintln!("[DEBUG] {refused:?}");
        std::process::exit(2);
    }
}
