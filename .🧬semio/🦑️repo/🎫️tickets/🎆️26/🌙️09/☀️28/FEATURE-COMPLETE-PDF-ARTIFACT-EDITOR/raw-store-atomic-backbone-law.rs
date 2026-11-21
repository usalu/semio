//! 🧪️ Exercises exact native Channel and Port queues with independently authored atomic frames.

use semio_framework_value::{AuthoringWallet, RetainedCloneGrant};
use spr_source_overlay::CausalPayload;
use store_backbone_overlay::{
    ActorBackboneChannelOwner, Backbone, Backbones, ChannelBackbone, PortBackbone,
    atomic_transport::{AtomicBackboneDelivery, AtomicBackboneReceive, take_atomic_front},
};
use std::{future::Future, task::{Context, Poll, Waker}};

const INPUT: &[u8] = include_bytes!("🗑️generated/raw-spr-transport/prepared");
const PNG: &[u8] = include_bytes!("🗑️generated/large-picked/large-picked.png");
const URI: &str = "temp://atomic-law";
const BINDING: [u8; 32] = [93; 32];
const TURN: RetainedCloneGrant = RetainedCloneGrant { maximum_items: 1 << 16, maximum_copy_bytes: 4096,
    maximum_capacity_bytes: 128 << 10, maximum_release_bytes: 128 << 10, maximum_depth: 64 };
const STOCK: RetainedCloneGrant = RetainedCloneGrant { maximum_items: 1 << 28, maximum_copy_bytes: 1 << 28,
    maximum_capacity_bytes: 1 << 27, maximum_release_bytes: 1 << 27, ..TURN };

fn block<T: Future>(future: T) -> T::Output {
    let mut future = std::pin::pin!(future);
    match future.as_mut().poll(&mut Context::from_waker(Waker::noop())) {
        Poll::Ready(value) => value,
        Poll::Pending => panic!("in-memory atomic transport blocked"),
    }
}

fn roundtrip(mut transport: Backbones, mut loopback: impl FnMut(), retry_target: Option<&mut Backbones>) -> usize {
    let original = CausalPayload::try_from_slice(INPUT).unwrap();
    let mut source = AtomicBackboneDelivery::begin(&original, BINDING).unwrap();
    let mut receiver = None;
    let mut wallet = AuthoringWallet::new(STOCK, 1 << 28);
    let mut frames = 0;
    let mut retry_target = retry_target;
    while !source.terminal_is_empty() {
        let loan = wallet.run(TURN, 1 << 16, |effect| source.advance(&original, effect)).unwrap().unwrap();
        wallet.post(loan.output.unwrap()).unwrap();
        if source.demands().unwrap() != Default::default() { continue; }
        if let Some(wrong) = retry_target.take() { assert!(block(source.send_ready(wrong)).is_err()); }
        if !block(source.send_ready(&mut transport)).unwrap() { continue; }
        loopback();
        assert!(block(transport.receive()).unwrap().is_empty());
        let mut handed = take_atomic_front(&transport).unwrap();
        assert!(handed.is_some());
        if receiver.is_none() {
            let (next, receipt) = wallet.run(TURN, 1 << 16, |effect| {
                AtomicBackboneReceive::admit(handed.as_ref().unwrap(), BINDING, effect)
            }).unwrap().unwrap().output.unwrap().unwrap();
            wallet.post(receipt).unwrap();
            receiver = Some(next);
        }
        let region = receiver.as_mut().unwrap();
        let denied = wallet.run(TURN, 1 << 16, |effect| region.push(&[0; 32], &mut handed, effect)).unwrap().unwrap();
        assert!(denied.output.is_err());
        let receipt = wallet.run(TURN, 1 << 16, |effect| region.push(&BINDING, &mut handed, effect)).unwrap().unwrap().output.unwrap();
        wallet.post(receipt).unwrap();
        assert!(handed.is_none());
        frames += 1;
    }
    let mut receiver = receiver.unwrap();
    let group = loop {
        let (value, receipt) = wallet.run(TURN, 1 << 16, |effect| receiver.seal(&BINDING, "fixture-document", "fixture-actor", effect))
            .unwrap().unwrap().output.unwrap();
        wallet.post(receipt).unwrap();
        if let Some(group) = value { break group; }
    };
    assert_eq!(group.edit_id(), "edit-png-82");
    assert_eq!(group.events().len(), 15);
    assert!(group.events().iter().flat_map(|event| event.diff.payload.iter()).eq(PNG.iter().copied()));
    for _ in 0..1_048_576 {
        if receiver.terminal_is_empty() { break; }
        let (turn, _) = wallet.run_settled(TURN, 1 << 16, |effect| receiver.close(4096, effect)).unwrap().unwrap();
        wallet.post(turn.result.unwrap().progress()).unwrap();
    }
    assert!(receiver.terminal_is_empty());
    frames
}

fn main() {
    let (channel, actor) = block(ChannelBackbone::pair(URI));
    let channel_frames = roundtrip(Backbones::Channel(channel), || {
        let message = actor.try_pop_front().unwrap().unwrap();
        block(actor.push(message)).unwrap();
    }, None);
    let (ports, actor) = ActorBackboneChannelOwner::pair(URI);
    let mut wrong = Backbones::Port(block(PortBackbone::with_channel("temp://foreign", ports.clone())));
    let port = block(PortBackbone::with_channel(URI, ports));
    let port_frames = roundtrip(Backbones::Port(port), || {
        let wire = actor.take_outbound().unwrap().unwrap();
        actor.push_inbound(URI, &wire).unwrap();
    }, Some(&mut wrong));
    actor.begin_retire().unwrap();
    assert!(actor.terminal_is_empty().unwrap());
    assert!(channel_frames > 1 && port_frames == channel_frames);
    #[cfg(inbox)]
    {
        continued_inbound();
        owned_actor_wire();
    }
    println!("[DEBUG] Real Channel and Port each sealed {port_frames} frames, matched every original byte, and retired the exact receive custody");
}

#[cfg(inbox)]
fn continued_inbound() {
    use store_backbone_overlay::atomic_transport::{AtomicBackboneInbox, AtomicInboxStep};
    use semio_framework_value::RetainedCloneProgress;

    let parts: &[&[u8]] = &[
        include_bytes!("🗑️generated/raw-spr-transport/0.part"), include_bytes!("🗑️generated/raw-spr-transport/1.part"),
        include_bytes!("🗑️generated/raw-spr-transport/2.part"), include_bytes!("🗑️generated/raw-spr-transport/3.part"),
        include_bytes!("🗑️generated/raw-spr-transport/4.part"), include_bytes!("🗑️generated/raw-spr-transport/5.part"),
        include_bytes!("🗑️generated/raw-spr-transport/6.part"), include_bytes!("🗑️generated/raw-spr-transport/7.part"),
    ];
    let mut wallet = AuthoringWallet::new(STOCK, 1 << 28);
    let mut inbox = AtomicBackboneInbox::default();
    let tiny = RetainedCloneGrant { maximum_copy_bytes: 0, ..TURN };
    for (i, bytes) in parts.iter().enumerate() {
        let mut input = Some(CausalPayload::try_from_slice(bytes).unwrap());
        let refused = wallet.run(tiny, 1 << 16, |effect| inbox.advance(&mut input, BINDING, "fixture-document", effect)).unwrap().unwrap().output.unwrap();
        assert!(matches!(refused, (AtomicInboxStep::Runnable, progress) if progress == RetainedCloneProgress::default()));
        assert!(input.is_some());
        for wake in 1..1024 {
            let (state, receipt) = wallet.run(TURN, 1 << 16, |effect| inbox.advance(&mut input, BINDING, "fixture-document", effect))
                .unwrap().unwrap().output.unwrap();
            wallet.post(receipt).unwrap();
            match state {
                AtomicInboxStep::Waiting => {
                    assert!(input.is_none() && i + 1 < parts.len() && inbox.ready().is_none());
                    break;
                }
                AtomicInboxStep::Runnable => assert!(wake < 1023),
                AtomicInboxStep::Ready => {
                    assert!(wake > 2 && input.is_none() && i + 1 == parts.len());
                    break;
                }
            }
        }
    }
    let edit = inbox.accept_ready().unwrap();
    assert!(edit.events().iter().flat_map(|row| row.diff.payload.iter()).eq(PNG.iter().copied()));
    let mut close_turns = 0;
    for _ in 0..1_048_576 {
        if inbox.original_frames_terminal() { break; }
        let (step, _) = wallet.run_settled(TURN, 1 << 16, |effect| inbox.close(effect)).unwrap().unwrap();
        wallet.post(step.result.unwrap().progress()).unwrap();
        close_turns += 1;
    }
    assert!(inbox.original_frames_terminal());
    let mut donated = Some(exact_hot_frame(parts[0]));
    let mut inbox = AtomicBackboneInbox::default();
    let armed = wallet.run(TURN, 1 << 16, |effect| inbox.take_hot(&mut donated, effect)).unwrap().unwrap().output.unwrap();
    assert!(donated.is_none() && armed.copied_items != 0);
    wallet.post(armed).unwrap();
    let (result, receipt) = wallet.run(TURN, 1 << 16, |effect| inbox.advance(&mut None, BINDING, "fixture-document", effect))
        .unwrap().unwrap().output.unwrap();
    assert!(matches!(result, AtomicInboxStep::Runnable));
    wallet.post(receipt).unwrap();
    let mut cancelled_turns = 0;
    for _ in 0..10_000 {
        if inbox.original_frames_terminal() { break; }
        let (step, _) = wallet.run_settled(TURN, 1 << 16, |effect| inbox.close(effect)).unwrap().unwrap();
        wallet.post(step.result.unwrap().progress()).unwrap();
        cancelled_turns += 1;
    }
    assert!(inbox.original_frames_terminal() && inbox.ready().is_none());
    println!("[DEBUG] The inbox waited for each incomplete group, verified without extra input, exposed one edit, and paid exact close in {close_turns} turns. Mid-frame cancellation retired in {cancelled_turns} turns.");
}

#[cfg(inbox)]
fn exact_hot_frame(part: &[u8]) -> Vec<u8> {
    let mut wire = vec![1, 1, 0, 1, 0, 8];
    let mut len = part.len();
    while len >= 128 {
        wire.push((len as u8 & 127) | 128);
        len >>= 7;
    }
    wire.push(len as u8);
    wire.extend_from_slice(part);
    wire
}

#[cfg(inbox)]
fn owned_actor_wire() {
    let mut wallet = AuthoringWallet::new(STOCK, 1 << 28);
    let prepared = CausalPayload::try_from_slice(INPUT).unwrap();
    let mut delivery = AtomicBackboneDelivery::begin(&prepared, BINDING).unwrap();
    let (ports, owner) = ActorBackboneChannelOwner::pair(URI);
    let target = Backbones::Port(block(PortBackbone::with_channel(URI, ports)));
    let mut count = 0;
    for _ in 0..8_192 {
        if delivery.terminal_is_empty() { break; }
        let receipt = if delivery.needs_actor_wire() {
            wallet.run(TURN, 1 << 16, |effect| delivery.advance_actor_wire(&target, effect)).unwrap().unwrap().output.unwrap()
        } else {
            wallet.run(TURN, 1 << 16, |effect| delivery.advance(&prepared, effect)).unwrap().unwrap().output.unwrap()
        };
        wallet.post(receipt).unwrap();
        if let Some(frame) = owner.take_outbound().unwrap() {
            let bytes: &[u8] = match count {
                0 => include_bytes!("🗑️generated/raw-spr-transport/0.part"),
                1 => include_bytes!("🗑️generated/raw-spr-transport/1.part"),
                2 => include_bytes!("🗑️generated/raw-spr-transport/2.part"),
                3 => include_bytes!("🗑️generated/raw-spr-transport/3.part"),
                4 => include_bytes!("🗑️generated/raw-spr-transport/4.part"),
                5 => include_bytes!("🗑️generated/raw-spr-transport/5.part"),
                6 => include_bytes!("🗑️generated/raw-spr-transport/6.part"),
                7 => include_bytes!("🗑️generated/raw-spr-transport/7.part"),
                _ => panic!("too many physical packets"),
            };
            assert_eq!(frame, exact_hot_frame(bytes));
            count += 1;
        }
    }
    assert!(delivery.terminal_is_empty());
    assert_eq!(count, 8);
    assert!(owner.take_outbound().unwrap().is_none());
    owner.begin_retire().unwrap();
    println!("[DEBUG] The mounted actor Port took eight exact owned wire frames under paid bounded output work");
}
