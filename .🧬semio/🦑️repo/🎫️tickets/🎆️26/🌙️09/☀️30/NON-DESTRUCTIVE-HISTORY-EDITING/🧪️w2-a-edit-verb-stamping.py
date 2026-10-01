"""🏷️ Stamps the authoring verb on every edit the plugin runtime publishes: the direct `Apply`/`AmendLast` dispatch, the
composed group dispatch, and both batched (artifact and config) publications."""
import pathlib

P = pathlib.Path("/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️.rs")
text = P.read_text()
pairs = [
    ("""            match self.store.dispatch(vcs_command).await {
                Ok(receipt) => self.record_dispatch_receipt(receipt),""", """            self.store.set_authoring_verb(Some(verb.to_string()));
            let dispatched = self.store.dispatch(vcs_command).await;
            self.store.set_authoring_verb(None);
            match dispatched {
                Ok(receipt) => self.record_dispatch_receipt(receipt),"""),
    ("""            let receipt = self.composition.dispatch_group(&parent_ref, &mut self.store, &mut dispatches, parent_ops, Vec::new(), group_meta).await.map_err(|error| plugin_sdk_fault(error.to_string()))?;""", """            self.store.set_authoring_verb(Some(verb.to_string()));
            let receipt = self.composition.dispatch_group(&parent_ref, &mut self.store, &mut dispatches, parent_ops, Vec::new(), group_meta).await;
            self.store.set_authoring_verb(None);
            let receipt = receipt.map_err(|error| plugin_sdk_fault(error.to_string()))?;"""),
    ("""                            Ok(mut publication) => {
                                publication.set_coalesce_key(emit.coalesce_key.take());
                                mounted.pending_artifact_publication""", """                            Ok(mut publication) => {
                                publication.set_coalesce_key(emit.coalesce_key.take());
                                publication.set_verb(Some(mounted.verb.clone()));
                                mounted.pending_artifact_publication"""),
    ("""                            Ok(mut publication) => {
                                publication.set_coalesce_key(emit.coalesce_key.clone());
                                mounted.pending_artifact_publication""", """                            Ok(mut publication) => {
                                publication.set_coalesce_key(emit.coalesce_key.clone());
                                publication.set_verb(Some(mounted.verb.clone()));
                                mounted.pending_artifact_publication"""),
]
for old, new in pairs:
    count = text.count(old)
    if count == 0 and text.count(new) == 1:
        continue
    assert count == 1, (count, old[:120])
    text = text.replace(old, new)
P.write_text(text)
print("ok")
