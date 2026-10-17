# Exact-Preservation Surface Fixture Active API Frontier

11 identified **active pre-existing Source API mismatches** remain exact under this packaging freeze. They are not certified compilable code. Parent requires an independent packaging audit before separate scenario-preserving API adaptation. No legacy overload/context minting/bypass. Three name-only close_step candidates were excluded because they have different actual receivers; method name alone is not a type oracle.

## Actual Defining Signature Identities

```json
{
  "path": "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧵️instance/📥️staging/🏗️ctx-builders/🦀️.rs",
  "name": "dispatch_typed",
  "expected": 4,
  "definitions": [
    {
      "line": 101,
      "signature": "pub fn dispatch_typed<'a, 'b, 'context>(&'a mut self, command: A::Command, meta: &'a ActionMeta, identity: &'a mut semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority<'b>,cx:&'a mut semio_framework_job::StepContext<'context>) -> impl Future<Output = Result<InvocationResult, Fault>> + 'a + use<'a, 'b, 'context, A, M> ",
      "sha256": "210ef37b65004727094abc0c5ad04e62c6662fbe447dc004b61ea5fe0cd233d4"
    }
  ]
}
```
```json
{
  "path": "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/⚛️reactor/🔌️app/🦀️.rs",
  "name": "consume_media",
  "expected": 3,
  "definitions": [
    {
      "line": 518,
      "signature": "async fn consume_media(&mut self, _port: &str, artifact: MediaArtifact, cx:&mut semio_framework_job::StepContext<'_>) -> Result<MediaConsumption, MediaArtifactError> ",
      "sha256": "5d4a24ac889a11d8a551e3dae59eb514db3e4bb5bef30877c5973f004ae2eb59"
    }
  ]
}
```
```json
{
  "path": "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/⚛️reactor/🔌️app/🦀️.rs",
  "name": "handle_action",
  "expected": 5,
  "definitions": [
    {
      "line": 224,
      "signature": "async fn handle_action(&mut self, action: &str, args: &mut Option<DslValue>, meta: &ActionMeta, identity: &mut semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority<'_>, cx:&mut semio_framework_job::StepContext<'_>) -> Result<Option<InvocationResult>,semio_framework_job::OriginalTurnFailure>;",
      "sha256": "5d7e0333d1d0e597acb2e34c8a2a52e3342b05c939e59c37f818d6c376e02265"
    }
  ]
}
```
```json
{
  "path": "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/⚛️reactor/🔌️app/🦀️.rs",
  "name": "export_media",
  "expected": 2,
  "definitions": [
    {
      "line": 468,
      "signature": "async fn export_media(&mut self, _port: &str,_cx:&mut semio_framework_job::StepContext<'_>) -> Result<Option<Media>,semio_framework_job::OriginalTurnFailure> ",
      "sha256": "216379e30a10644b093a089ea799ea59e0cbf9979106af297c1ec3678e7632d0"
    }
  ]
}
```
```json
{
  "path": "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📜️declaration/👁️view/🛬️lifted/👷️job/🦀️.rs",
  "name": "close_step",
  "expected": 2,
  "definitions": [
    {
      "line": 274,
      "signature": "pub fn close_step(&self,cx:&mut StepContext<'_>,loan:RetainedCloneGrant)->Result<PluginLifecycleStep,PluginLifecycleFailure>",
      "sha256": "9a944d1c2a681bc3b934800dde1220d149e8a71c3072db6d4cdf64c40b46b442"
    }
  ]
}
```

## Actual Stale Calls

```json
{
  "path": "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧪️tests/🧬️mutation-fixtures-surface/🦀️.rs",
  "line": 699,
  "law": "editor_fixture_still_mutates_normally",
  "name": "dispatch_typed",
  "args": 3,
  "expected": 4,
  "expression": "app.dispatch_typed(SurfaceEditorCommand::Increment, &meta(\"local\"), &mut crate::app_testing::artifact_app_laws::fixture_identity())"
}
```
```json
{
  "path": "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧪️tests/🧬️mutation-fixtures-surface/🦀️.rs",
  "line": 728,
  "law": "registered_editor_consumes_intrinsic_natural_bytes_through_the_real_media_route",
  "name": "consume_media",
  "args": 2,
  "expected": 3,
  "expression": "app.consume_media(crate::app_contract::NATURAL_FILE_PORT, artifact)"
}
```
```json
{
  "path": "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧪️tests/🧬️mutation-fixtures-surface/🦀️.rs",
  "line": 959,
  "law": "editor_fixture_without_a_manifest_declaration_fails_closed",
  "name": "dispatch_typed",
  "args": 3,
  "expected": 4,
  "expression": "app.dispatch_typed(SurfaceEditorCommand::Increment, &meta(\"local\"), &mut crate::app_testing::artifact_app_laws::fixture_identity())"
}
```
```json
{
  "path": "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧪️tests/🧬️mutation-fixtures-surface/🦀️.rs",
  "line": 1058,
  "law": "viewer_rejects_every_contract_mutating_verb",
  "name": "handle_action",
  "args": 4,
  "expected": 5,
  "expression": "app.handle_action(verb, None, &meta(\"local\"), &mut crate::app_testing::artifact_app_laws::fixture_identity())"
}
```
```json
{
  "path": "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧪️tests/🎞️media-owner-context/🦀️.rs",
  "line": 46,
  "law": "close_owner",
  "name": "close_step",
  "args": 1,
  "expected": 2,
  "expression": "owner.close_step(semio_framework_value::retained_clone::RetainedCloneGrant { maximum_items: items, ..grant })"
}
```
```json
{
  "path": "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧪️tests/🎞️media-owner-context/🦀️.rs",
  "line": 135,
  "law": "media_export_request_context_preserves_exact_supplied_owner",
  "name": "export_media",
  "args": 1,
  "expected": 2,
  "expression": "editor.export_media(context_port)",
  "oracle": "exact literal call inside macro token body, manually checked against actual receiver/type"
}
```
```json
{
  "path": "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧪️tests/🎞️media-owner-context/🦀️.rs",
  "line": 136,
  "law": "media_export_request_context_preserves_exact_supplied_owner",
  "name": "export_media",
  "args": 1,
  "expected": 2,
  "expression": "viewer.export_media(context_port)",
  "oracle": "exact literal call inside macro token body, manually checked against actual receiver/type"
}
```
```json
{
  "path": "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧪️tests/🎞️media-owner-context/🦀️.rs",
  "line": 139,
  "law": "media_export_request_context_preserves_exact_supplied_owner",
  "name": "export_media",
  "args": 1,
  "expected": 2,
  "expression": "editor.export_media(default_port)",
  "oracle": "exact literal call inside macro token body, manually checked against actual receiver/type"
}
```
```json
{
  "path": "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧪️tests/🎞️media-owner-context/🦀️.rs",
  "line": 140,
  "law": "media_export_request_context_preserves_exact_supplied_owner",
  "name": "export_media",
  "args": 1,
  "expected": 2,
  "expression": "viewer.export_media(default_port)",
  "oracle": "exact literal call inside macro token body, manually checked against actual receiver/type"
}
```
```json
{
  "path": "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧪️tests/🎞️media-owner-context/🦀️.rs",
  "line": 141,
  "law": "media_export_request_context_preserves_exact_supplied_owner",
  "name": "export_media",
  "args": 1,
  "expected": 2,
  "expression": "editor.export_media(\"unknown:out\")",
  "oracle": "exact literal call inside macro token body, manually checked against actual receiver/type"
}
```
```json
{
  "path": "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧪️tests/🎞️media-owner-context/🦀️.rs",
  "line": 142,
  "law": "media_export_request_context_preserves_exact_supplied_owner",
  "name": "export_media",
  "args": 1,
  "expected": 2,
  "expression": "viewer.export_media(\"unknown:out\")",
  "oracle": "exact literal call inside macro token body, manually checked against actual receiver/type"
}
```

## Required Next Scenario Mapping

- Two dispatch_typed calls: use same already-live OriginalFixtureCaller cx fourth parameter, keep positive Artifact publication and registry-less refusal assertions.
- Natural consume_media: use same cx third parameter and preserve intrinsic registered media/load scenario.
- Viewer handle_action: borrow actual &mut Option<DslValue> and same cx, preserve every rejected mutating verb and existing authoritative fault reason.
- MediaOwner close_owner: actual handle.close_step takes(cx,loan); retain zero-items refusal, preserve exact original paid owner/context, trace subsequent physical retirement turns before terminal-empty. Immediate Complete may be obsolete; no whole-owner release shortcut.
- Six live media exports: actual Result<Option<Media>,OriginalTurnFailure>, so preserve original supplied/default/unknown-port scenario, real Option pending/completion and retained-domain error ownership. Do not replace exact NotImplemented intent with broad is_err.

Actual API/lifetime corrections are outside this exact-body packaging freeze. This is scoped Source evidence, not exhaustive Rust type/privacy/runtime verification; no Rust compilation performed.
