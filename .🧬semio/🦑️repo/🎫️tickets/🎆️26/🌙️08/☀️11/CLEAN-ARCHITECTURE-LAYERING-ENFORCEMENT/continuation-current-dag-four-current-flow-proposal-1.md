# Current Flow DAG Four Proposal

DAG3 current Flow before advanced through exact endpoint port admission, preserving its two document-schema literals. Fresh DAG4 derives the complete current Flow before/after and changes only those two schema literals. New concurrent exact declared-port checks and function are preserved byte-for-byte. Rust fields, methods and dialect identity remain unchanged. All14 other intended full pairs and6context/6schema/36neutral escape controls are conserved; full current preimages are checked by the actual script. No Root writes/native/compiler.

Current Flow SHA 0309524f62ef6ae5d0bca51985b1da335aa172e74348ece38fb758e86e3a0290; after SHA 874f837b1a77460c54ee5f195a931613234c2193ccc01480a4ee08a99679c533.

```diff
--- 
+++ 
@@ -330,7 +330,7 @@
             geometry_port: None,
             operator_registry: None,
             host_snapshot: host_snapshot,
-            dag: DagHost::from_host_snapshot(DagHostSnapshot { schema: "dag.host_snapshot".into(), camera: semio_framework_artifact_infinite_dag::DagCamera { x: 0.0, y: 0.0, zoom: 1.0 }, nodes: vec![], edges: vec![] }),
+            dag: DagHost::from_host_snapshot(DagHostSnapshot { schema: "dag.hostDocument".into(), camera: semio_framework_artifact_infinite_dag::DagCamera { x: 0.0, y: 0.0, zoom: 1.0 }, nodes: vec![], edges: vec![] }),
             outputs: BTreeMap::new(),
             export_payloads: BTreeMap::new(),
             last_eval_json: String::new(),
@@ -1964,7 +1964,7 @@
                 Some(DagHostSnapshotEdge { id: syn.id.clone(), source: format!("{}@{}", syn.from, from_port), target: format!("{}@{}", syn.to, to_port), route_style: EdgeRouteStyle::default(), properties: PropertyBag::new() })
             })
             .collect();
-        DagHostSnapshot { schema: "dag.host_snapshot".into(), camera: semio_framework_artifact_infinite_dag::DagCamera { x: self.host_snapshot.camera.x, y: self.host_snapshot.camera.y, zoom: self.host_snapshot.camera.zoom }, nodes, edges }
+        DagHostSnapshot { schema: "dag.hostDocument".into(), camera: semio_framework_artifact_infinite_dag::DagCamera { x: self.host_snapshot.camera.x, y: self.host_snapshot.camera.y, zoom: self.host_snapshot.camera.zoom }, nodes, edges }
     }
 
     fn screen_to_world_point(&self, sx: f64, sy: f64) -> canvas::Point {

```
