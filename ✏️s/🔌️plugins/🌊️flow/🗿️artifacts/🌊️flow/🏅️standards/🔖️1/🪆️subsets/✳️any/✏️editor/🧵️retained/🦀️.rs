//! 🧵️ Flow-owned byte frontiers for retained preparation and retirement.

use super::{FlowConfig, FlowConfigMutation, FlowMutation};
use flow::{neural, FlowGui, FlowLayoutEntry, FlowNodeGui, FlowPreviewGui, NodeChrome, Widget};
use std::collections::LinkedList;
use std::mem::ManuallyDrop;
use flow::retained::{FlowOwner, FlowRetirement};
use store::ErasedSnapshotRetirement;

#[path = "🎚️config/🦀️.rs"]
pub(super) mod config;

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
    Value(neural::Value),
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
    Config(FlowConfig),
    ConfigMutation(FlowConfigMutation),
    Scene(super::FlowWorkingScene),
    Widgets(Vec<Widget>),
    Specs(Vec<flow::SynapseSpec>),
    Layouts(flow::OrderedMap<flow::WidgetLayout>),
    Chrome(NodeChrome),
}

#[derive(Default)]
pub(super) struct Retirement {
    owners: ManuallyDrop<LinkedList<Owner>>,
}

impl Drop for Retirement {
    fn drop(&mut self) { if !std::thread::panicking() { assert!(self.is_empty(), "Flow app retirement must reach terminal-empty before drop"); } }
}

impl Retirement {
    fn domain(&mut self, owner: FlowOwner) { let mut retirement = FlowRetirement::default(); retirement.push(owner); self.push(Owner::Domain(retirement)); }
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
                if !values.is_empty() { self.push(Owner::Strings(values)); }
                if let Some(value) = next { self.text(value); }
            }
            Owner::Set(value) => self.domain(FlowOwner::Set(value)),
            Owner::Dictionary(value) => self.domain(FlowOwner::Dictionary(value)),
            Owner::Domain(mut owner) => {
                match owner.close_step(maximum_items, maximum_bytes).expect("typed Flow retirement") {
                    store::SnapshotRetirementStep::Pending { released_bytes: bytes, .. } => released_bytes = bytes,
                    store::SnapshotRetirementStep::Complete => {},
                    store::SnapshotRetirementStep::Blocked => unreachable!("positive Flow retirement grant"),
                }
                if !owner.is_empty() { self.push(Owner::Domain(owner)); }
            }
            Owner::Value(neural::Value::Dictionary(value)) => self.push(Owner::Dictionary(value)),
            Owner::Value(neural::Value::Atom(neural::Atom::String(value))) => self.text(value),
            Owner::Value(neural::Value::Atom(_)) => {}
            Owner::Widget(widget) => self.widget(widget),
            Owner::Scene(value) => self.push(Owner::Domain(crate::artifacts::flow::retirement::retire_scene(value))),
            Owner::Widgets(mut values) => {
                let next = values.pop();
                if !values.is_empty() { self.push(Owner::Widgets(values)); }
                if let Some(value) = next { self.push(Owner::Widget(value)); }
            }
            Owner::Specs(mut values) => {
                let next = values.pop();
                if !values.is_empty() { self.push(Owner::Specs(values)); }
                if let Some(value) = next {
                    self.text(value.id); self.text(value.from); self.text(value.to); self.text(value.from_port); self.text(value.to_port);
                }
            }
            Owner::Layouts(value) => self.domain(FlowOwner::Layouts(value)),
            Owner::Chrome(value) => match value {
                NodeChrome::Note { text } => self.text(text),
                NodeChrome::Image { src } => self.text(src),
                NodeChrome::Variable { name, schema } => { self.text(name); self.text(schema); }
                NodeChrome::Plain { .. } => {}
                NodeChrome::Slider { label, .. } => self.text(label),
            },
            Owner::Tree(tree) => {
                self.push(Owner::Neurons(tree.neurons));
                self.push(Owner::Synapses(tree.synapses));
            }
            Owner::Neurons(mut values) => {
                let next = values.pop();
                if !values.is_empty() { self.push(Owner::Neurons(values)); }
                if let Some(value) = next {
                    self.text(value.id);
                    self.text(value.kind);
                    self.push(Owner::Dictionary(value.params));
                    if let Some(tree) = value.tree { self.push(Owner::Tree(*tree)); }
                }
            }
            Owner::Synapses(mut values) => {
                let next = values.pop();
                if !values.is_empty() { self.push(Owner::Synapses(values)); }
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
                if !values.is_empty() { self.push(Owner::Previews(values)); }
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
                if !values.is_empty() { self.push(Owner::Layout(values)); }
                if let Some(value) = next { self.text(value.id); }
            }
            Owner::Mutation(value) => self.push(Owner::Domain(crate::artifacts::flow::retirement::retire_mutation(value))),
            Owner::Mutations(mut values) => {
                let next = values.pop();
                if !values.is_empty() { self.push(Owner::Mutations(values)); }
                if let Some(value) = next { self.push(Owner::Mutation(value)); }
            }
            Owner::Config(value) => {
                self.push(Owner::Strings(value.preview_off_node_ids));
                self.text(value.lod_mode);
                self.text(value.catalogue_sections_json);
                self.text(value.automation_enabled_json);
                self.text(value.contributions_json);
                self.text(value.generation_json);
                self.text(value.duplicate_widget_progress_json);
                self.text(value.locale);
            }
            Owner::ConfigMutation(value) => match value {
                FlowConfigMutation::Snapshot { config } => self.push(Owner::Config(config)),
                FlowConfigMutation::SetPreviewOff { node_ids } => self.push(Owner::Strings(node_ids)),
                FlowConfigMutation::SetLodMode { value } | FlowConfigMutation::SetLocale { value } => self.text(value),
                FlowConfigMutation::SetContributions { json }
                | FlowConfigMutation::SetAutomationEnabled { json }
                | FlowConfigMutation::SetGeneration { json }
                | FlowConfigMutation::SetDuplicateWidgetProgress { json }
                | FlowConfigMutation::SetCatalogueSections { sections_json: json } => self.text(json),
                FlowConfigMutation::SetCamera { .. } | FlowConfigMutation::SetProximityDistance { .. }
                | FlowConfigMutation::SetGridVisible { .. } | FlowConfigMutation::SetGridSnapEnabled { .. }
                | FlowConfigMutation::SetGridFactor { .. } | FlowConfigMutation::CancelDuplicateWidget { .. } => {}
            },
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
            Widget::InputSlider { id, label, .. } => { self.text(id); self.text(label); }
            Widget::InputNote { id, text } => { self.text(id); self.text(text); }
            Widget::InputImage { id, src } => { self.text(id); self.text(src); }
            Widget::Variable { id, name, schema } => { self.text(id); self.text(name); self.text(schema); }
            Widget::OutputPreview { id, preview, expanded } => {
                self.text(id);
                self.push(Owner::Dictionary(preview));
                self.push(Owner::Set(expanded));
            }
            Widget::OutputAction { id, action } => { self.text(id); self.text(action); }
            Widget::OutputExport { id, format } => { self.text(id); self.text(format); }
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

impl store::ErasedSnapshotRetirement for Retirement {
    fn close_step(&mut self, maximum_items: usize, maximum_bytes: usize) -> Result<store::SnapshotRetirementStep, String> {
        Ok(match self.step(maximum_items, maximum_bytes) {
            semio_framework_job::InteractiveJobCloseStep::Pending { released_items, released_bytes } => store::SnapshotRetirementStep::Pending { released_items, released_bytes },
            semio_framework_job::InteractiveJobCloseStep::Complete => store::SnapshotRetirementStep::Complete,
            semio_framework_job::InteractiveJobCloseStep::Blocked => store::SnapshotRetirementStep::Blocked,
        })
    }
    fn terminal_is_empty(&self) -> bool { self.is_empty() }
}

//#region 🎚️ConfigCopy
pub(super) struct ConfigSource<'a> {
    preview: &'a [String],
    text: [&'a str; 7],
    camera: &'a flow::CameraJson,
    proximity: f64,
    visible: bool,
    snap: bool,
    factor: f64,
}

impl<'a> ConfigSource<'a> {
    pub(super) fn base(config: &'a FlowConfig) -> Self {
        Self {
            preview: &config.preview_off_node_ids,
            text: [&config.lod_mode, &config.catalogue_sections_json, &config.automation_enabled_json, &config.contributions_json, &config.generation_json, &config.duplicate_widget_progress_json, &config.locale],
            camera: &config.camera,
            proximity: config.proximity_distance,
            visible: config.grid_visible,
            snap: config.grid_snap_enabled,
            factor: config.grid_factor,
        }
    }

    pub(super) fn post(base: &'a FlowConfig, mutation: &'a FlowConfigMutation, cancel_matches: bool) -> Self {
        let mut source = Self::base(base);
        match mutation {
            FlowConfigMutation::Snapshot { config } => return Self::base(config),
            FlowConfigMutation::SetPreviewOff { node_ids } => source.preview = node_ids,
            FlowConfigMutation::SetCamera { camera } => source.camera = camera,
            FlowConfigMutation::SetLodMode { value } => source.text[0] = value,
            FlowConfigMutation::SetProximityDistance { value } => source.proximity = *value,
            FlowConfigMutation::SetGridVisible { value } => source.visible = *value,
            FlowConfigMutation::SetGridSnapEnabled { value } => source.snap = *value,
            FlowConfigMutation::SetGridFactor { value } => source.factor = *value,
            FlowConfigMutation::SetCatalogueSections { sections_json } => source.text[1] = sections_json,
            FlowConfigMutation::SetAutomationEnabled { json } => source.text[2] = json,
            FlowConfigMutation::SetContributions { json } => source.text[3] = json,
            FlowConfigMutation::SetGeneration { json } => source.text[4] = json,
            FlowConfigMutation::SetDuplicateWidgetProgress { json } => source.text[5] = json,
            FlowConfigMutation::CancelDuplicateWidget { .. } if cancel_matches => source.text[5] = "",
            FlowConfigMutation::CancelDuplicateWidget { .. } => {}
            FlowConfigMutation::SetLocale { value } => source.text[6] = value,
        }
        source
    }
}

pub(super) struct ConfigCopy {
    target: Option<FlowConfig>,
    field: usize,
    item: usize,
    bytes: Vec<u8>,
    selected: Option<usize>,
    preview_reserved: bool,
    utf8_remaining: u8,
    utf8_min: u8,
    utf8_max: u8,
}

impl ConfigCopy {
    pub(super) fn new(source: &ConfigSource<'_>, selected: Option<usize>) -> Self {
        Self {
            target: Some(FlowConfig {
                preview_off_node_ids: Vec::new(), camera: source.camera.clone(), lod_mode: String::new(), proximity_distance: source.proximity,
                grid_visible: source.visible, grid_snap_enabled: source.snap, grid_factor: source.factor,
                catalogue_sections_json: String::new(), automation_enabled_json: String::new(), contributions_json: String::new(),
                generation_json: String::new(), duplicate_widget_progress_json: String::new(), locale: String::new(),
            }),
            field: selected.unwrap_or(0), item: 0, bytes: Vec::new(), selected, preview_reserved: false,
            utf8_remaining: 0, utf8_min: 0x80, utf8_max: 0xbf,
        }
    }

    pub(super) fn complete(&self) -> bool {
        self.field >= 8
    }

    /// 🧬️ Validates each copied UTF-8 byte once, so completed buffers need no whole-string rescan.
    pub(super) fn step(&mut self, source: &ConfigSource<'_>, maximum_bytes: usize) -> Result<usize, String> {
        if maximum_bytes == 0 || self.complete() { return Ok(0); }
        if self.field == 0 && !self.preview_reserved {
            self.target.as_mut().ok_or("Flow config copy lost target")?.preview_off_node_ids.try_reserve_exact(source.preview.len()).map_err(|_| "Flow preview vector allocation failed")?;
            self.preview_reserved = true;
            return Ok(0);
        }
        let text = if self.field == 0 {
            match source.preview.get(self.item) {
                Some(value) => value.as_str(),
                None => { self.next_field(); return Ok(0); }
            }
        } else {
            source.text[self.field - 1]
        };
        if self.bytes.is_empty() && self.bytes.capacity() < text.len() {
            self.bytes.try_reserve_exact(text.len()).map_err(|_| "Flow config text allocation failed")?;
            return Ok(0);
        }
        let start = self.bytes.len();
        let count = maximum_bytes.min(text.len().saturating_sub(start));
        for &byte in &text.as_bytes()[start..start + count] {
            self.validate_utf8(byte)?;
        }
        self.bytes.extend_from_slice(&text.as_bytes()[start..start + count]);
        if self.bytes.len() == text.len() {
            if self.utf8_remaining != 0 { return Err("Flow copy ended inside a UTF-8 scalar".into()); }
            let value = unsafe { String::from_utf8_unchecked(std::mem::take(&mut self.bytes)) };
            let target = self.target.as_mut().ok_or_else(|| "Flow config copy lost its target".to_owned())?;
            match self.field {
                0 => { target.preview_off_node_ids.push(value); self.item += 1; }
                1 => target.lod_mode = value,
                2 => target.catalogue_sections_json = value,
                3 => target.automation_enabled_json = value,
                4 => target.contributions_json = value,
                5 => target.generation_json = value,
                6 => target.duplicate_widget_progress_json = value,
                7 => target.locale = value,
                _ => unreachable!(),
            }
            if self.field != 0 { self.next_field(); }
        }
        Ok(count)
    }

    fn validate_utf8(&mut self, byte: u8) -> Result<(), String> {
        if self.utf8_remaining > 0 {
            if !(self.utf8_min..=self.utf8_max).contains(&byte) { return Err("Flow copy received invalid UTF-8 continuation".into()); }
            self.utf8_remaining -= 1;
            self.utf8_min = 0x80;
            self.utf8_max = 0xbf;
        } else {
            match byte {
                0..=0x7f => {}
                0xc2..=0xdf => self.utf8_remaining = 1,
                0xe0 => { self.utf8_remaining = 2; self.utf8_min = 0xa0; }
                0xe1..=0xec | 0xee..=0xef => self.utf8_remaining = 2,
                0xed => { self.utf8_remaining = 2; self.utf8_max = 0x9f; }
                0xf0 => { self.utf8_remaining = 3; self.utf8_min = 0x90; }
                0xf1..=0xf3 => self.utf8_remaining = 3,
                0xf4 => { self.utf8_remaining = 3; self.utf8_max = 0x8f; }
                _ => return Err("Flow copy received invalid UTF-8 lead byte".into()),
            }
        }
        Ok(())
    }

    fn next_field(&mut self) {
        self.field = if self.selected.is_some() { 8 } else { self.field + 1 };
        self.item = 0;
    }

    pub(super) fn take(&mut self) -> Option<FlowConfig> {
        if self.complete() { self.target.take() } else { None }
    }

    pub(super) fn retire(mut self, retirement: &mut Retirement) {
        if let Some(target) = self.target.take() { retirement.push(Owner::Config(target)); }
        retirement.push(Owner::Bytes(std::mem::take(&mut self.bytes)));
    }
}

pub(super) fn inverse_field(mutation: &FlowConfigMutation) -> Option<usize> {
    match mutation {
        FlowConfigMutation::Snapshot { .. } => None,
        FlowConfigMutation::SetPreviewOff { .. } => Some(0),
        FlowConfigMutation::SetLodMode { .. } => Some(1),
        FlowConfigMutation::SetCatalogueSections { .. } => Some(2),
        FlowConfigMutation::SetAutomationEnabled { .. } => Some(3),
        FlowConfigMutation::SetContributions { .. } => Some(4),
        FlowConfigMutation::SetGeneration { .. } => Some(5),
        FlowConfigMutation::SetDuplicateWidgetProgress { .. } | FlowConfigMutation::CancelDuplicateWidget { .. } => Some(6),
        FlowConfigMutation::SetLocale { .. } => Some(7),
        _ => Some(8),
    }
}

pub(super) fn inverse(mutation: &FlowConfigMutation, config: FlowConfig) -> FlowConfigMutation {
    match mutation {
        FlowConfigMutation::Snapshot { .. } => FlowConfigMutation::Snapshot { config },
        FlowConfigMutation::SetPreviewOff { .. } => FlowConfigMutation::SetPreviewOff { node_ids: config.preview_off_node_ids },
        FlowConfigMutation::SetCamera { .. } => FlowConfigMutation::SetCamera { camera: config.camera },
        FlowConfigMutation::SetLodMode { .. } => FlowConfigMutation::SetLodMode { value: config.lod_mode },
        FlowConfigMutation::SetProximityDistance { .. } => FlowConfigMutation::SetProximityDistance { value: config.proximity_distance },
        FlowConfigMutation::SetGridVisible { .. } => FlowConfigMutation::SetGridVisible { value: config.grid_visible },
        FlowConfigMutation::SetGridSnapEnabled { .. } => FlowConfigMutation::SetGridSnapEnabled { value: config.grid_snap_enabled },
        FlowConfigMutation::SetGridFactor { .. } => FlowConfigMutation::SetGridFactor { value: config.grid_factor },
        FlowConfigMutation::SetCatalogueSections { .. } => FlowConfigMutation::SetCatalogueSections { sections_json: config.catalogue_sections_json },
        FlowConfigMutation::SetAutomationEnabled { .. } => FlowConfigMutation::SetAutomationEnabled { json: config.automation_enabled_json },
        FlowConfigMutation::SetContributions { .. } => FlowConfigMutation::SetContributions { json: config.contributions_json },
        FlowConfigMutation::SetGeneration { .. } => FlowConfigMutation::SetGeneration { json: config.generation_json },
        FlowConfigMutation::SetDuplicateWidgetProgress { .. } | FlowConfigMutation::CancelDuplicateWidget { .. } => FlowConfigMutation::SetDuplicateWidgetProgress { json: config.duplicate_widget_progress_json },
        FlowConfigMutation::SetLocale { .. } => FlowConfigMutation::SetLocale { value: config.locale },
    }
}
//#endregion 🎚️ConfigCopy

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;
    use semio_framework_job::InteractiveJobCloseStep as Step;
    use semio_framework_plugin::retained_command::ArtifactCommandWork;

    #[test]
    fn config_copy_and_targeted_inverse_obey_maximum_text_fixture() {
        let fixture: serde_json::Value = serde_json::from_str(include_str!("../🧪️fixtures/🧫️grant-frontier/🔣️.json")).unwrap();
        for row in fixture["cases"].as_array().unwrap() {
            let text = row["unit"].as_str().unwrap().repeat(row["repetitions"].as_u64().unwrap() as usize);
            let grant = row["grantBytes"].as_u64().unwrap() as usize;
            let base = FlowConfig::default();
            let mutation = FlowConfigMutation::SetCatalogueSections { sections_json: text.clone() };
            let source = ConfigSource::post(&base, &mutation, false);
            let mut copy = ConfigCopy::new(&source, None);
            let mut copied = 0;
            for _ in 0..text.len() + 100 {
                let count = copy.step(&source, grant).unwrap();
                assert!(count <= grant);
                copied += count;
                if copy.complete() { break; }
            }
            let post = copy.take().unwrap();
            assert_eq!(post.catalogue_sections_json, text);
            assert_eq!(post.locale, base.locale);
            assert_eq!(copied, super::super::flow_config_text_bytes(&post));
            let source = ConfigSource::base(&base);
            let mut undo = ConfigCopy::new(&source, inverse_field(&mutation));
            for _ in 0..100 {
                undo.step(&source, grant).unwrap();
                if undo.complete() { break; }
            }
            assert_eq!(inverse(&mutation, undo.take().unwrap()), FlowConfigMutation::SetCatalogueSections { sections_json: base.catalogue_sections_json });
        }
    }

    #[test]
    fn direct_preview_close_finishes_with_production_grant_and_exact_bytes() {
        for (value, grant) in [("tiny".to_owned(), 4096), ("🌊".repeat(4096), 4096), ("🌊".repeat(4096), 1)] {
            let expected = value.len();
            let mut work = super::super::FlowDirectStoreWork::new("setPreviewOff");
            work.preview_off = Some(vec![value]);
            work.begin_close();
            let mut released = 0;
            for _ in 0..expected + 10 {
                match work.close_step(1, grant) {
                    Step::Pending { released_items, released_bytes } => {
                        assert!(released_items <= 1);
                        assert!(released_bytes <= grant);
                        released += released_bytes;
                    }
                    Step::Complete => break,
                    other => panic!("unexpected direct close result: {other:?}"),
                }
            }
            assert!(work.terminal_is_empty());
            assert_eq!(released, expected);
        }
    }

    #[test]
    fn restore_never_drops_live_retained_owners() {
        let mut work = super::super::FlowDirectStoreWork::new("setPreviewOff");
        work.preview_off = Some(vec!["owned".into()]);
        assert!(work.restore(&[0; 17]).is_err());
        assert_eq!(work.preview_off.as_ref().unwrap(), &["owned"]);
        work.begin_close();
        for _ in 0..20 {
            if matches!(work.close_step(1, 1), Step::Complete) { break; }
        }
        assert!(work.terminal_is_empty());
    }

    #[test]
    fn retirement_obeys_language_neutral_grants_and_releases_exact_bytes() {
        let fixture: serde_json::Value = serde_json::from_str(include_str!("../🧪️fixtures/🧫️grant-frontier/🔣️.json")).unwrap();
        for row in fixture["cases"].as_array().unwrap() {
            let text = row["unit"].as_str().unwrap().repeat(row["repetitions"].as_u64().unwrap() as usize);
            let expected = row["expectedTextBytes"].as_u64().unwrap() as usize;
            let grant = row["grantBytes"].as_u64().unwrap() as usize;
            let mut retirement = Retirement::default();
            retirement.push(Owner::Strings(vec![text]));
            assert!(matches!(retirement.step(0, grant), Step::Blocked));
            assert!(matches!(retirement.step(1, 0), Step::Blocked));
            let mut released = 0;
            let mut steps = 0;
            loop {
                steps += 1;
                assert!(steps < expected + 10);
                match retirement.step(1, grant) {
                    Step::Pending { released_items, released_bytes } => {
                        assert!(released_items <= 1);
                        assert!(released_bytes <= grant);
                        released += released_bytes;
                    }
                    Step::Complete => break,
                    other => panic!("unexpected retirement result: {other:?}"),
                }
            }
            assert_eq!(released, expected);
            assert!(retirement.is_empty());
        }
    }

    #[test]
    fn nested_dictionary_moves_without_cloning_and_retires_at_one_byte() {
        let text = "🌊".repeat(4096);
        let dictionary = neural::Dictionary::new().insert("key", neural::Value::Dictionary(neural::Dictionary::new().insert("nested", neural::Value::Atom(neural::Atom::String(text)))));
        let mut retirement = Retirement::default();
        retirement.push(Owner::Dictionary(dictionary));
        let mut bytes = 0;
        for _ in 0..16410 {
            match retirement.step(1, 1) {
                Step::Pending { released_bytes, .. } => bytes += released_bytes,
                Step::Complete => break,
                other => panic!("unexpected retirement result: {other:?}"),
            }
        }
        assert!(retirement.is_empty());
        assert_eq!(bytes, 16384 + 3 + 6);
    }
}
//#endregion 🧪️Tests
