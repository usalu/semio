import sys,os
sys.path.insert(0,os.path.dirname(os.path.abspath(__file__)))
import edit
from edit import *
FW="🧰️framework/🛍️products/💻️os/🔨️modules/"
ST=FW+"🏪️store/"
PL=FW+"🔌️plugin/"
# checkpoint-order unused imports
p=ST+"🕰️history/🏁️replay/checkpoint-order.rs"
sub(p,"use semio_framework_os_spr as protocol;\nuse semio_framework_os_spr::{OpBinary,OpText};\n","")
sub(p,"use semio_framework_value::{FromValue,RetainedCloneGrant,RetainedCloneProgress,ToValue,ValueError};","use semio_framework_value::{RetainedCloneGrant,RetainedCloneProgress,ValueError};")
# checkout.rs
p=ST+"🕰️history/🏁️replay/checkout.rs"
sub(p,"let order=self.order.as_ref().unwrap().borrow_order()?;let parts=store.parts_ref();return self.inverses.close_demands_in_order","let order=self.order.as_ref().ok_or_else(||invalid(\"checkpoint checkout lost its original order while closing\"))?.borrow_order()?;let parts=store.parts_ref();return self.inverses.close_demands_in_order")
sub(p,"let order=self.order.as_ref().unwrap().borrow_order()?;let parts=store.parts();close_original_retained_step","let order=self.order.as_ref().ok_or_else(||invalid(\"checkpoint checkout lost its original order while closing\"))?.borrow_order()?;let parts=store.parts();close_original_retained_step")
sub(p,"assert!(self.gate.as_mut().unwrap().abort());Ok(RetainedCloneStep::Progress(progress(d)))","if !self.gate.as_mut().ok_or_else(||invalid(\"checkpoint checkout lost its visibility gate while closing\"))?.abort(){return Err(invalid(\"checkpoint checkout could not abort its visibility gate\"))}Ok(RetainedCloneStep::Progress(progress(d)))")
sub(p,"FactoryAuthority::new(self.view.take().unwrap())","FactoryAuthority::new(self.view.take().ok_or_else(||invalid(\"checkpoint checkout lost its visibility view while closing\"))?)")
sub(p,"Some(self.gate.take().unwrap().into_retirement())","Some(self.gate.take().ok_or_else(||invalid(\"checkpoint checkout lost its visibility gate while closing\"))?.into_retirement())")
# media-transfer
p=PL+"🧵️instance/🔌️plugin-app-impl/🎞️media-transfer/🦀️.rs"
LOST='ValueError::literal(semio_framework_value::ValueRefusalKind::InvariantViolated,'
sub(p,"let original=self.media.media_download_preparation.as_mut().unwrap();","let original=self.media.media_download_preparation.as_mut().ok_or_else(||"+LOST+"\"download publication lost its retained preparation\"))?;")
sub(p,"&mut original.buffers.as_mut().unwrap()[original.stage];","&mut original.buffers.as_mut().ok_or_else(||"+LOST+"\"download metadata lost its copy buffers\"))?[original.stage];")
sub(p,"let buffers=original.buffers.as_mut().unwrap();let document=String::from_utf8(std::mem::take(&mut buffers[0])).expect(\"copied original document remains UTF-8\");let mime_type=String::from_utf8(std::mem::take(&mut buffers[1])).expect(\"copied original MIME remains UTF-8\");","let buffers=original.buffers.as_mut().ok_or_else(||"+LOST+"\"download metadata lost its copy buffers\"))?;let document=String::from_utf8(std::mem::take(&mut buffers[0])).map_err(|_|"+LOST+"\"copied original document lost its UTF-8 encoding\"))?;let mime_type=String::from_utf8(std::mem::take(&mut buffers[1])).map_err(|_|"+LOST+"\"copied original MIME lost its UTF-8 encoding\"))?;")
sub(p,"insert_admitted(operation_id,original.output.take().unwrap());","insert_admitted(operation_id,original.output.take().ok_or_else(||"+LOST+"\"download publication lost its donated output\"))?);")
# original/
p=PL+"🧵️instance/🧱️construction/📦️original/🦀️.rs"
sub(p,"use semio_framework_value::{RetirementDemand,ValueError,retained_clone::{RetainedCloneGrant,RetainedCloneProgress}};","use semio_framework_value::{RetirementDemand,ValueError,ValueRefusalKind,retained_clone::{RetainedCloneGrant,RetainedCloneProgress}};")
sub(p," fn finish_step(&self,"," fn closed()->ValueError{ValueError::literal(ValueRefusalKind::InvariantViolated,\"closed original App cannot continue its close ladder\")}\n fn finish_step(&self,")
sub(p,"PluginApp::close_step(self.original.as_deref_mut().unwrap(),cx,grant)?","PluginApp::close_step(self.original.as_deref_mut().ok_or_else(Self::closed)?,cx,grant)?")
sub(p,"let original=self.original.as_deref_mut().unwrap();let first=PluginApp::mounted_owner_close_step","let original=self.original.as_deref_mut().ok_or_else(Self::closed)?;let first=PluginApp::mounted_owner_close_step")
(None if os.environ.get("DRY") else flush())
print("DONE_B")
