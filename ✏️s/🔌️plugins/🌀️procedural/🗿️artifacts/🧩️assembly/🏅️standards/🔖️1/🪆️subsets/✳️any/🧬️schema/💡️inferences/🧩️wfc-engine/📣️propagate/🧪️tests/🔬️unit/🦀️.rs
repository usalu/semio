
use super::*;

#[test]
fn push_dedups_and_fifo_orders() {
    let mut q = PropQueue::new(4);
    q.push(NodeId(1));
    q.push(NodeId(2));
    q.push(NodeId(1)); // dedup
    assert_eq!(q.pop(), Some(NodeId(1)));
    assert_eq!(q.pop(), Some(NodeId(2)));
    assert_eq!(q.pop(), None);
}

#[test]
fn popped_node_can_be_repushed() {
    let mut q = PropQueue::new(2);
    q.push(NodeId(0));
    q.pop();
    q.push(NodeId(0));
    assert_eq!(q.pop(), Some(NodeId(0)));
}

#[test]
fn clear_resets_membership() {
    let mut q = PropQueue::new(2);
    q.push(NodeId(0));
    q.clear();
    assert!(q.is_empty());
    q.push(NodeId(0));
    assert_eq!(q.pop(), Some(NodeId(0)));
}

#[test]
fn push_all_enqueues_every_node_once() {
    let mut q = PropQueue::new(3);
    q.push_all(3);
    let mut seen = Vec::new();
    while let Some(n) = q.pop() {
        seen.push(n);
    }
    assert_eq!(seen, vec![NodeId(0), NodeId(1), NodeId(2)]);
}
