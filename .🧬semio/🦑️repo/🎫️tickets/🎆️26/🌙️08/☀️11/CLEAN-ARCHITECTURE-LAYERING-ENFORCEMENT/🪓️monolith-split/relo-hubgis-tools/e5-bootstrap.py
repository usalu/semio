"""🏗️ Registers the GIS Map inference binding at the hub composition root."""
import sys,os
sys.path.insert(0,os.path.dirname(os.path.abspath(__file__)))
from edit import Edit
H="/Users/ueli/Documents/semio/🌎️hub/"
e=Edit(H+'🏗️bootstrap/🦀️.rs')
e.rep('use semio_hub::inference::verified_gis_map_binding;\n','use semio_hub::inference::{verified_gis_map_binding, MapInferenceProposalBindingV1};\nuse semio_hub_gis::inference::GisMapInferenceBindingV1;\n')
e.rep('''        let gis_map_binding = match verified_catalog.as_ref() {
            Some(catalog) => verified_gis_map_binding(catalog.clone()).map_err''','''        let gis_map_inference_binding: Arc<dyn MapInferenceProposalBindingV1> = Arc::new(GisMapInferenceBindingV1);
        let gis_map_binding = match verified_catalog.as_ref() {
            Some(catalog) => verified_gis_map_binding(catalog.clone(), gis_map_inference_binding.as_ref()).map_err''')
e.rep('''                let committer = Arc::new(RetainedGisMapApprovalCommitterV1::new(db.clone(), db.storage().await, ledger.clone(), publisher));
                let runtime = Arc::new(HubInferenceRuntimeV1::new(binding.clone(), ledger, committer));''','''                let committer: Arc<RetainedGisMapApprovalCommitterV1<GisMapInferenceBindingV1>> = Arc::new(RetainedGisMapApprovalCommitterV1::new(db.clone(), db.storage().await, ledger.clone(), publisher));
                let runtime = Arc::new(HubInferenceRuntimeV1::new(binding.clone(), ledger, committer, gis_map_inference_binding.clone()));''')
e.save()
e=Edit(H+'🧪️tests/🔬️bin-unit/🦀️.rs')
e.rep('''    state.inference_runtime = Some(Arc::new(HubInferenceRuntimeV1::new(profile.binding().clone(), Arc::new(ledger), Arc::new(UnavailableGisMapApprovalCommitterV1))));''','''    state.inference_runtime = Some(Arc::new(HubInferenceRuntimeV1::new(profile.binding().clone(), Arc::new(ledger), Arc::new(UnavailableGisMapApprovalCommitterV1), Arc::new(GisMapInferenceBindingV1))));''')
e.rep('''    restarted.inference_runtime = Some(Arc::new(HubInferenceRuntimeV1::new(bound.profile.binding().clone(), Arc::new(reopened), Arc::new(UnavailableGisMapApprovalCommitterV1)''','''    restarted.inference_runtime = Some(Arc::new(HubInferenceRuntimeV1::new(bound.profile.binding().clone(), Arc::new(reopened), Arc::new(UnavailableGisMapApprovalCommitterV1), Arc::new(GisMapInferenceBindingV1)''')
e.save()
e=Edit(H+'🧪️tests/🔏️trusted-catalog-profile/🦀️.rs')
e.rep('''    let binding = crate::inference::verified_gis_map_binding(catalog.clone())''','''    let binding = crate::inference::verified_gis_map_binding(catalog.clone(), &semio_hub_gis::inference::GisMapInferenceBindingV1)''')
e.save()
e=Edit(H+'🗿️artifact-authority/🔏️trusted-catalog/🧪️tests/🔬️unit/🦀️.rs')
e.rep('''            let result = crate::inference::verified_gis_map_binding(catalog.clone());''','''            let result = crate::inference::verified_gis_map_binding(catalog.clone(), &semio_hub_gis::inference::GisMapInferenceBindingV1);''')
e.save()
print("bootstrap ok")
