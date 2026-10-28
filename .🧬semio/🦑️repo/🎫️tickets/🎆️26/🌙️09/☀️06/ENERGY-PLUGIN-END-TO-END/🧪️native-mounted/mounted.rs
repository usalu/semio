pub use semio_framework_plugin_instance_app::original_instance_stores;
#[path="../../../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧵️instance/🌱️initialization/🧪️tests/🪟️native-window/🦀️.rs"]mod fixture;
#[global_allocator]static HEAP:semio_framework_trace::HeapWitness=semio_framework_trace::HeapWitness;
use semio_framework_plugin_instance_app::{VcsArtifactApp,original_instance_admission::{OriginalInstanceServices,OriginalArtifactInstanceAdmission}};
use semio_framework_os_kernel::NoMembers;
#[test]fn actual_native_instance_and_registration_frames_fit_the_mounted_policy(){
 type App=fixture::NativeWindowApp;type Services=OriginalInstanceServices<App,NoMembers>;type Admission=OriginalArtifactInstanceAdmission<App,NoMembers>;
 let demands=[Services::birth_demands().unwrap(),Admission::birth_demands(),Admission::candidate_birth_demands()];
 assert!(demands.iter().all(|d|d.copy_bytes<=32768),"actual native service/admission/candidate quotes {demands:?}, services={}, admission={}, app={}",std::mem::size_of::<Services>(),std::mem::size_of::<Admission>(),std::mem::size_of::<VcsArtifactApp<App,NoMembers>>());
}

#[test]fn actual_single_window_registration_frame_fits_the_mounted_policy(){type Registration=semio_framework_plugin_instance_app::original_instance_admission::OriginalSingleWindowRegistration<fixture::NativeWindowApp,NoMembers,fixture::Owner>;let demand=Registration::birth_demands();assert!(demand.copy_bytes<=32768,"actual original registration quote {demand:?}");}

#[path="../../../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧵️instance/🌱️initialization/🎟️registration/🧪️tests/🦀️.rs"]mod original_registration_tests;
