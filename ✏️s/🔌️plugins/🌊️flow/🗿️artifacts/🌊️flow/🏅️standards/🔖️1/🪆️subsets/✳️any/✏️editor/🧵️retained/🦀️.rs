//! 🧵️ Flow-owned byte frontiers for retained preparation and retirement.

use super::FlowMutation;
use flow::neural;
use semio_framework_artifact_flow_flow::retained::{FlowOwner, FlowRetirement};
use semio_framework_artifact_flow_flow::{FlowGui, FlowLayoutEntry, FlowNodeGui, FlowPreviewGui, Widget};
use std::collections::LinkedList;
use std::mem::ManuallyDrop;
use store::ErasedSnapshotRetirement;

#[path = "🗿️artifact/🦀️.rs"]
pub(super) mod artifact;

#[path = "🧾️canonical/🦀️.rs"]
mod canonical;

#[path = "🔤️bytes/🦀️.rs"]
pub(super) mod bytes;

//#region 🧹️Retirement
pub(super) enum Owner {
    Bytes(Vec<u8>),
    Strings(Vec<String>),
    Set(flow::OrderedSet),
    Domain(FlowRetirement),
    Dictionary(neural::Dictionary),
    Widget(Widget),
    Tree(neural::Tree),
    Neurons(Vec<neural::Neuron>),
    Synapses(Vec<neural::Synapse>),
    Gui(FlowGui),
    Nodes(flow::OrderedMap<FlowNodeGui>),
    Previews(Vec<FlowPreviewGui>),
    Layout(Vec<FlowLayoutEntry>),
    Mutation(FlowMutation),
    Mutations(Vec<FlowMutation>),
    Scene(super::FlowWorkingScene),
}

#[derive(Default)]
pub(super) struct Retirement {
    owners: ManuallyDrop<LinkedList<Owner>>,
}

impl Drop for Retirement {
    fn drop(&mut self) {
        if !std::thread::panicking() {
            assert!(self.is_empty(), "Flow app retirement must reach terminal-empty before drop");
        }
    }
}

impl Retirement {
    fn domain(&mut self, owner: FlowOwner) {
        let mut retirement = FlowRetirement::default();
        retirement.push(owner);
        self.push(Owner::Domain(retirement));
    }
    pub(super) fn push(&mut self, owner: Owner) {
        self.owners.push_front(owner);
    }

    fn text(&mut self, value: String) {
        self.push(Owner::Bytes(value.into_bytes()));
    }

    pub(super) fn is_empty(&self) -> bool {
        self.owners.is_empty()
    }

    pub(super) fn step(&mut self, maximum_items: usize, maximum_bytes: usize) -> semio_framework_job::InteractiveJobCloseStep {
        use semio_framework_job::InteractiveJobCloseStep as Step;
        if self.is_empty() {
            return Step::Complete;
        }
        if maximum_items == 0 || maximum_bytes == 0 {
            return Step::Blocked;
        }
        let mut released_bytes = 0;
        match self.owners.pop_front().expect("nonempty retirement") {
            Owner::Bytes(mut bytes) => {
                released_bytes = maximum_bytes.min(bytes.len());
                bytes.truncate(bytes.len() - released_bytes);
                if !bytes.is_empty() {
                    self.push(Owner::Bytes(bytes));
                }
            }
            Owner::Strings(mut values) => {
                let next = values.pop();
                if !values.is_empty() {
                    self.push(Owner::Strings(values));
                }
                if let Some(value) = next {
                    self.text(value);
                }
            }
            Owner::Set(value) => self.domain(FlowOwner::Set(value)),
            Owner::Dictionary(value) => self.domain(FlowOwner::Dictionary(value)),
            Owner::Domain(mut owner) => {
                match owner.close_step(maximum_items, maximum_bytes).expect("typed Flow retirement") {
                    store::SnapshotRetirementStep::Pending { released_bytes: bytes, .. } => released_bytes = bytes,
                    store::SnapshotRetirementStep::Complete => {}
                    store::SnapshotRetirementStep::Blocked => unreachable!("positive Flow retirement grant"),
                }
                if !owner.is_empty() {
                    self.push(Owner::Domain(owner));
                }
            }
            Owner::Widget(widget) => self.widget(widget),
            Owner::Scene(value) => self.push(Owner::Domain(crate::retirement::retire_scene(value))),
            Owner::Tree(tree) => {
                self.push(Owner::Neurons(tree.neurons));
                self.push(Owner::Synapses(tree.synapses));
            }
            Owner::Neurons(mut values) => {
                let next = values.pop();
                if !values.is_empty() {
                    self.push(Owner::Neurons(values));
                }
                if let Some(value) = next {
                    self.text(value.id);
                    self.text(value.kind);
                    self.push(Owner::Dictionary(value.params));
                    if let Some(tree) = value.tree {
                        self.push(Owner::Tree(*tree));
                    }
                }
            }
            Owner::Synapses(mut values) => {
                let next = values.pop();
                if !values.is_empty() {
                    self.push(Owner::Synapses(values));
                }
                if let Some(value) = next {
                    self.text(value.id);
                    self.text(value.from);
                    self.text(value.to);
                    self.text(value.from_port);
                    self.text(value.to_port);
                }
            }
            Owner::Gui(value) => {
                self.push(Owner::Nodes(value.nodes));
                self.push(Owner::Previews(value.previews));
            }
            Owner::Nodes(value) => self.domain(FlowOwner::Nodes(value)),
            Owner::Previews(mut values) => {
                let next = values.pop();
                if !values.is_empty() {
                    self.push(Owner::Previews(values));
                }
                if let Some(value) = next {
                    self.text(value.id);
                    self.text(value.mode);
                    self.push(Owner::Dictionary(value.preview));
                    self.push(Owner::Set(value.expanded));
                    if let Some(source) = value.source {
                        self.text(source.neuron);
                        self.text(source.channel);
                    }
                }
            }
            Owner::Layout(mut values) => {
                let next = values.pop();
                if !values.is_empty() {
                    self.push(Owner::Layout(values));
                }
                if let Some(value) = next {
                    self.text(value.id);
                }
            }
            Owner::Mutation(value) => self.push(Owner::Domain(crate::retirement::retire_mutation(value))),
            Owner::Mutations(mut values) => {
                let next = values.pop();
                if !values.is_empty() {
                    self.push(Owner::Mutations(values));
                }
                if let Some(value) = next {
                    self.push(Owner::Mutation(value));
                }
            }
        }
        Step::Pending { released_items: 1, released_bytes }
    }

    fn widget(&mut self, widget: Widget) {
        match widget {
            Widget::Neuron { id, neuron_kind, params, input_ports, output_ports, .. } => {
                self.text(id);
                self.text(neuron_kind);
                self.push(Owner::Dictionary(params));
                self.push(Owner::Strings(input_ports));
                self.push(Owner::Strings(output_ports));
            }
            Widget::InputSlider { id, label, .. } => {
                self.text(id);
                self.text(label);
            }
            Widget::InputNote { id, text } => {
                self.text(id);
                self.text(text);
            }
            Widget::InputImage { id, src } => {
                self.text(id);
                self.text(src);
            }
            Widget::Variable { id, name, schema } => {
                self.text(id);
                self.text(name);
                self.text(schema);
            }
            Widget::OutputPreview { id, preview, expanded } => {
                self.text(id);
                self.push(Owner::Dictionary(preview));
                self.push(Owner::Set(expanded));
            }
            Widget::OutputAction { id, action } => {
                self.text(id);
                self.text(action);
            }
            Widget::OutputExport { id, format } => {
                self.text(id);
                self.text(format);
            }
            Widget::Cluster { id, name, tree, flow } => {
                self.text(id);
                self.text(name);
                self.push(Owner::Tree(tree));
                self.push(Owner::Gui(flow));
            }
        }
    }
}
//#endregion 🧹️Retirement

impl ErasedSnapshotRetirement for Retirement {
    fn close_step(&mut self, maximum_items: usize, maximum_bytes: usize) -> Result<store::SnapshotRetirementStep, String> {
        Ok(match self.step(maximum_items, maximum_bytes) {
            semio_framework_job::InteractiveJobCloseStep::Pending { released_items, released_bytes } => store::SnapshotRetirementStep::Pending { released_items, released_bytes },
            semio_framework_job::InteractiveJobCloseStep::Complete => store::SnapshotRetirementStep::Complete,
            semio_framework_job::InteractiveJobCloseStep::Blocked => store::SnapshotRetirementStep::Blocked,
        })
    }
    fn terminal_is_empty(&self) -> bool {
        self.is_empty()
    }
}

//#region 🧪️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🧪️Tests
