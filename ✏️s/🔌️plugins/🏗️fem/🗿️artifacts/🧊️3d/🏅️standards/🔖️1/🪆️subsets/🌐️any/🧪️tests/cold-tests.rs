//! 🧪️ Cold persistent rows refuse aliased text and drain without allocating.
use semio_s_artifact_fem_3d_model::standards::v1::subsets::any::schema::snapshot::cold;
use semio_s_fem_material::volumetric::FemMaterialValue;
use semio_s_fem_node::volumetric::FemNodeValue;

#[test]
fn nested_string_alias_refuses_the_unchanged_cold_tree(){
 let mut nodes=cold::nodes(vec![FemNodeValue{id:"alpha".into(),x:1.,y:2.,z:3.}]).unwrap();
 let alias=nodes.get(0).unwrap().id.clone();
 let (ok,attempts)=semio_framework_value_testing_allocator::refuse_one_allocation_on_this_thread(||nodes.cold_dismantle());assert!(!ok);assert_eq!(attempts,0);assert_eq!(nodes.get(0).unwrap().id.as_str(),"alpha");drop(alias);
 let (ok,attempts)=semio_framework_value_testing_allocator::refuse_one_allocation_on_this_thread(||nodes.cold_dismantle());assert!(ok);assert_eq!(attempts,0);assert!(nodes.is_empty());
}

#[test]
fn material_text_alias_refuses_unchanged_then_drains_without_allocation(){
 let mut materials=cold::materials(vec![FemMaterialValue{id:"steel".into(),name:"Steel".into(),e:2.,nu:0.2,rho:1.,g:1.}]).unwrap();
 let alias=materials.get(0).unwrap().name.clone();
 let (ok,attempts)=semio_framework_value_testing_allocator::refuse_one_allocation_on_this_thread(||materials.cold_dismantle());assert!(!ok);assert_eq!(attempts,0);assert_eq!(materials.get(0).unwrap().name.as_str(),"Steel");drop(alias);
 let (ok,attempts)=semio_framework_value_testing_allocator::refuse_one_allocation_on_this_thread(||materials.cold_dismantle());assert!(ok);assert_eq!(attempts,0);assert!(materials.is_empty());
}
