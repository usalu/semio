"""📜️ Re-points the hub script gates at the GIS-owned binding module the moved store, factory and executable code now lives in."""
import sys,os
sys.path.insert(0,os.path.dirname(os.path.abspath(__file__)))
from edit import Edit
e=Edit("/Users/ueli/Documents/semio/🌎️hub/📦️packages/🦀️rust/📜️script.ts")
e.rep('''  const runtime = readModuleTreeSource(join(repoRoot, "🌎️hub", "💡️inference", "🏃️runtime", "🦀️.rs"));
  for (const symbol of [
    "RetainedGisMapApprovalCommitterV1",''','''  const runtime = readModuleTreeSource(join(repoRoot, "🌎️hub", "💡️inference", "🏃️runtime", "🦀️.rs"));
  const gisBinding = readModuleTreeSource(join(repoRoot, "🌎️hub", "🧩️compositions", "🌍️gis", "💡️inference", "🦀️.rs"));
  if (runtime.includes("semio_s_artifact_") || !runtime.includes("impl<B: ComposedMapCommitBindingV1> RetainedGisMapApprovalCommitterV1<B>"))
    throw new Error("the retained committer names an artifact crate instead of staying generic over the registered composed-map binding");
  for (const symbol of ["impl ComposedMapCommitBindingV1 for GisMapInferenceBindingV1", "impl MapInferenceProposalBindingV1 for GisMapInferenceBindingV1", "gis_map_parent_stamped_one_item_preparation_factory"])
    if (!gisBinding.includes(symbol)) throw new Error(`GIS Map inference binding source is missing ${symbol}`);
  for (const symbol of [
    "RetainedGisMapApprovalCommitterV1",''')
e.rep('''    "RetainedGisMapDocumentStateV1::Publishing",
    "gis_map_parent_stamped_one_item_preparation_factory",
    "sole committed WAL event",''','''    "RetainedGisMapDocumentStateV1::Publishing",
    "B::parent_preparation_factory(",
    "sole committed WAL event",''')
e.rep('''  const storeConstruction = retainedCommitter.slice(retainedCommitter.indexOf("fn parent_store("), retainedCommitter.indexOf("fn derived_children("));''','''  const storeConstruction = gisBinding.slice(gisBinding.indexOf("fn parent_store("), gisBinding.indexOf("pub fn gis_map_region_preview("));''')
e.rep('''    !retainedCommitter.includes("let drawing_id = snapshot.drawing.child_id.clone();") ||
    !retainedCommitter.includes("let value_id = snapshot.value.child_id.clone();") ||''','''    !gisBinding.includes("let drawing_id = snapshot.drawing.child_id.clone();") ||
    !gisBinding.includes("let value_id = snapshot.value.child_id.clone();") ||
    !retainedCommitter.includes("B::member_stores(&key, snapshot, actor)") ||''')
e.rep('''  const catalog = readFileSync(join(repoRoot, "🌎️hub", "💡️inference", "📇️catalog", "🦀️.rs"), "utf8");
  const trusted''','''  const catalog = readFileSync(join(repoRoot, "🌎️hub", "💡️inference", "📇️catalog", "🦀️.rs"), "utf8");
  const gisBinding = readFileSync(join(repoRoot, "🌎️hub", "🧩️compositions", "🌍️gis", "💡️inference", "🦀️.rs"), "utf8");
  if (!gisBinding.includes("semio_s_artifact_gis_gismap::gis_map_inference_service()")) throw new Error("the GIS Map inference binding no longer registers the literal native executable");
  const trusted''')
e.rep('''    "selected_document_open",
    "gis_map_inference_service",
    "executable_identity",''','''    "selected_document_open",
    "proposals.inference_service()",
    "executable_identity",''')
e.save()
print("script ok")
