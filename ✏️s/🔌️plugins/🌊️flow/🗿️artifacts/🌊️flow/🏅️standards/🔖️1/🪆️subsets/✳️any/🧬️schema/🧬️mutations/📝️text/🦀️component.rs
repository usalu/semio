//! ⚡️ Flow artifact — Op facet re-exports `FlowMutation`.
pub use crate::artifacts::flow::schema::mutations::{apply_flow_mutation, inverse_flow_mutation, FlowMutation};

//#region 📖️SemioGrammar
pub const COMPONENT_GRAMMAR_SEMIO: &str = include_str!("📖️component.grammar.semio");
pub const COMPONENT_GRAMMAR_PATH: &str = concat!(module_path!(), "::📖️component.grammar.semio");
//#endregion 📖️SemioGrammar

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;
    use crate::artifacts::flow::FlowSnapshot;
    use protocol::{Identified, Mutation, MutationDiff};

    #[test]
    fn move_widgets_inverse_restores_base() {
        let base = FlowSnapshot::default();
        let mutation = FlowMutation::MoveWidgets(crate::artifacts::flow::schema::mutations::move_widgets::mutation::MoveWidgets {
            entries: vec![flow::FlowLayoutEntry { id: "slider".into(), layout: Some(flow::WidgetLayout { x: 10.0, y: 20.0 }) }],
        });
        let forward = mutation.diff(&base).apply(&base).expect("valid mutation diff");
        assert_eq!(forward.to_fixture().layout.get("slider"), Some(&flow::WidgetLayout { x: 10.0, y: 20.0 }));
        let restored = mutation.inverse(&base).iter().fold(forward, |snapshot, inverse| {
            inverse.diff(&snapshot).apply(&snapshot).expect("valid mutation diff")
        });
        assert_eq!(restored, base);
    }

    #[test]
    fn create_widget_then_delete_widget_round_trips_to_base() {
        let base = FlowSnapshot::default();
        let widget = flow::Widget::InputNote { id: "note-1".into(), text: "hello".into() };
        let create = FlowMutation::CreateWidget(crate::artifacts::flow::schema::mutations::create_widget::mutation::CreateWidget { index: base.to_fixture().widgets.len(), widget });
        let after_create = create.diff(&base).apply(&base).expect("valid mutation diff");
        assert!(after_create.to_fixture().widgets.iter().any(|widget| widget.id() == "note-1"));

        let delete = FlowMutation::DeleteWidget(crate::artifacts::flow::schema::mutations::delete_widget::mutation::DeleteWidget { id: "note-1".into() });
        let after_delete = delete.diff(&after_create).apply(&after_create).expect("valid mutation diff");
        assert_eq!(after_delete, base);

        let restored = delete.inverse(&after_create).iter().fold(after_delete, |snapshot, inverse| inverse.diff(&snapshot).apply(&snapshot).expect("valid mutation diff"));
        assert_eq!(restored, after_create);
    }

    #[test]
    fn connect_widgets_then_disconnect_widgets_round_trips_to_base() {
        let base = FlowSnapshot::default();
        let connect = FlowMutation::ConnectWidgets(crate::artifacts::flow::schema::mutations::connect_widgets::mutation::ConnectWidgets {
            index: base.to_fixture().synapses.len(),
            id: "s3".into(),
            from: "slider".into(),
            from_port: "number".into(),
            to: "add".into(),
            to_port: "b".into(),
        });
        let after_connect = connect.diff(&base).apply(&base).expect("valid mutation diff");
        assert!(after_connect.to_fixture().synapses.iter().any(|synapse| synapse.id == "s3"));

        let disconnect = FlowMutation::DisconnectWidgets(crate::artifacts::flow::schema::mutations::disconnect_widgets::mutation::DisconnectWidgets { id: "s3".into() });
        let after_disconnect = disconnect.diff(&after_connect).apply(&after_connect).expect("valid mutation diff");
        assert_eq!(after_disconnect, base);

        let restored = disconnect.inverse(&after_connect).iter().fold(after_disconnect, |snapshot, inverse| inverse.diff(&snapshot).apply(&snapshot).expect("valid mutation diff"));
        assert_eq!(restored, after_connect);
    }

    /// 🌉️ The composite pilot: `duplicate-widget` plans `create-widget` then `connect-widgets` —
    /// applying it must land the same widget/synapse pair a hand-written create+connect would, and
    /// its `inverse` (folded from the SAME plan) must undo both in one shot.
    #[test]
    fn duplicate_widget_composite_round_trips_to_base() {
        let base = FlowSnapshot::default();
        let widget = flow::Widget::InputNote { id: "note-1".into(), text: "hello".into() };
        let create = FlowMutation::CreateWidget(crate::artifacts::flow::schema::mutations::create_widget::mutation::CreateWidget { index: base.to_fixture().widgets.len(), widget });
        let after_create = create.diff(&base).apply(&base).expect("valid mutation diff");

        let duplicate = FlowMutation::DuplicateWidget(crate::artifacts::flow::schema::mutations::duplicate_widget::mutation::DuplicateWidget {
            source_id: "note-1".into(),
            new_id: "note-2".into(),
            synapse_id: "note-1-to-note-2".into(),
            from_port: "out".into(),
            to_port: "in".into(),
        });
        let after_duplicate = duplicate.diff(&after_create).apply(&after_create).expect("valid mutation diff");
        assert!(after_duplicate.to_fixture().widgets.iter().any(|widget| widget.id() == "note-2"));
        assert!(after_duplicate.to_fixture().synapses.iter().any(|synapse| synapse.id == "note-1-to-note-2"));

        let restored = duplicate.inverse(&after_create).iter().fold(after_duplicate, |snapshot, inverse| inverse.diff(&snapshot).apply(&snapshot).expect("valid mutation diff"));
        assert_eq!(restored, after_create);
    }
}
//#endregion 🧪️Tests
