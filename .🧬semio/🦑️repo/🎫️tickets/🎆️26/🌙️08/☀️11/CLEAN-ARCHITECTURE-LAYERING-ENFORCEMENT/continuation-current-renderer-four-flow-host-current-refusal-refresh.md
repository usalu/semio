# Current Renderer4 Flow Host Authority Refresh

Green1 actually exited Nx1 before controls: Flow host advanced from 3d1d3f465e36765f691e5fa859b7d5595c84ae0a6d5f421d5483ff25a71c79cb to 0309524f62ef6ae5d0bca51985b1da335aa172e74348ece38fb758e86e3a0290. Its raw log, refused full proposal/helper and current host body are retained in generated/current-engine-layout-4/green-1. No admission receipt was emitted.

The successor binds the full current Flow host authority. This is a source authority refresh only: no physical layout acceptance, compiler execution, historical restoration or Root write. Descriptor values remain the measured Product7 proposals, requiring fresh owning native verification.

Actual Product11 initial-to-current full-body source difference:

```diff
--- Product11 initial current authority
+++ current Root host authority
@@ -951,6 +951,12 @@
         }
         if !widget_has_input(to_id, &self.host_snapshot.widgets, &self.host_snapshot.synapses, &self.kind_infos) {
             return Err(FlowCoreError::NoInputPort(to_id.to_string()));
+        }
+        if !widget_has_declared_port(from_id, from_port, PortSide::Output, &self.host_snapshot.widgets, &self.host_snapshot.synapses, &self.kind_infos) {
+            return Err(FlowCoreError::UnknownOutputPort(from_port.to_string()));
+        }
+        if !widget_has_declared_port(to_id, to_port, PortSide::Input, &self.host_snapshot.widgets, &self.host_snapshot.synapses, &self.kind_infos) {
+            return Err(FlowCoreError::UnknownInputPort(to_port.to_string()));
         }
         let existing: Vec<(String, String)> = self.host_snapshot.synapses.iter().map(|s| (s.from.clone(), s.to.clone())).collect();
         if would_create_cycle(&existing, from_id, to_id) {
@@ -4786,7 +4792,7 @@
 }
 
 /// 🔤️ The value schemas one endpoint declares, read straight off the port the operator catalogue
-/// published — an undeclared port answers with an empty list, which stays connectable.
+/// published — an untyped declared port answers with an empty list; connection admission separately checks exact endpoint identity.
 pub fn widget_port_value_types(widget_id: &str, port_id: &str, side: PortSide, widgets: &[Widget], synapses: &[SynapseSpec], kind_infos: &HashMap<String, OperatorInfo>) -> Vec<String> {
     let Some(widget) = widgets.iter().find(|widget| widget_id_for(widget) == widget_id) else {
         return Vec::new();
@@ -4806,6 +4812,13 @@
         return true;
     }
     source.iter().any(|provided| target.iter().any(|accepted| accepted == provided))
+}
+
+fn widget_has_declared_port(widget_id: &str, port_id: &str, side: PortSide, widgets: &[Widget], synapses: &[SynapseSpec], kind_infos: &HashMap<String, OperatorInfo>) -> bool {
+    widgets.iter().find(|widget| widget_id_for(widget) == widget_id).is_some_and(|widget| {
+        let (inputs, outputs, _, _) = widget_io_ports(widget, synapses, kind_infos);
+        (if side == PortSide::Output { outputs } else { inputs }).iter().any(|port| port.id == port_id)
+    })
 }
 
 fn widget_has_output(widget_id: &str, widgets: &[Widget], synapses: &[SynapseSpec], kind_infos: &HashMap<String, OperatorInfo>) -> bool {

```
