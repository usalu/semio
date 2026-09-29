"""🩺️ FH1 family A — runtime cleanup faults name their literal code (`RuntimeCleanupFault::fault_code`, one arm per
vector row right beside the table), OS frame refusals take a `FaultCode`, and a plugin whose bundle failed to
assemble refuses with the framework's `plugin.assembly-failed` (the assembly error stays developer detail)."""
import sys
sys.path.insert(0, "/Users/ueli/Documents/semio/.tmp-ticket/wp-fh1")
from fh1_edit import apply
P = "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️.rs"
ROWS = [("Clock", "clock"), ("ClockRegression", "clock-regression"), ("InteractiveCeiling", "interactive-ceiling"), ("ZeroProgress", "zero-progress"), ("AlreadyClosing", "already-closing"), ("AdmissionRejected", "admission-rejected"), ("Resume", "resume"), ("PriorOutcome", "prior-outcome"), ("MissingSession", "missing-session"), ("AbiMismatch", "abi-mismatch"), ("Checkout", "checkout"), ("CheckedOutJob", "checked-out-job"), ("TakeOutcome", "take-outcome"), ("CooperativeClock", "clock-cooperative"), ("OwnerPoisoned", "owner-poisoned"), ("InstancePoisoned", "instance-poisoned"), ("InstanceNotDrained", "instance-not-drained"), ("MaintenancePoisoned", "maintenance-poisoned"), ("MaintenanceNotDrained", "maintenance-not-drained")]
arms = "\n".join(f'                Self::{variant} => FaultCode::new("plugin.internal.{code}"),' for variant, code in ROWS)
apply("", [
    (P, '''        fn from_index(index: u8) -> Option<Self> {
            RUNTIME_CLEANUP_FAULTS.get(index as usize).copied()
        }
    }''', '''        fn from_index(index: u8) -> Option<Self> {
            RUNTIME_CLEANUP_FAULTS.get(index as usize).copied()
        }

        /// 🧯️ The vector's `code` as the literal the fault census reads — one arm per row of
        /// [`RUNTIME_CLEANUP_FAULT_VECTORS`], held equal by the cleanup fault-vector law.
        fn fault_code(self) -> FaultCode {
            match self {
''' + arms + '''
            }
        }
    }'''),
    (P, '''        let vector = cause.vector();
        let elapsed = if elapsed_us == RUNTIME_CLEANUP_UNMEASURED_US { "unmeasured".to_string() } else { format!("{elapsed_us}us") };
        Fault::new(
            FaultOrigin::Plugin,
            FaultCode::new(vector.code),''', '''        let vector = cause.vector();
        let elapsed = if elapsed_us == RUNTIME_CLEANUP_UNMEASURED_US { "unmeasured".to_string() } else { format!("{elapsed_us}us") };
        Fault::new(
            FaultOrigin::Plugin,
            cause.fault_code(),'''),
    (P, '''    async fn push_os_fault(frames: &mut Vec<protocol::AppFrame>, in_reply_to: Option<u64>, code: &'static str, message: String) {
        push_app_fault(frames, in_reply_to, Fault::new(FaultOrigin::Os, FaultCode::new(code), message)).await;''', '''    async fn push_os_fault(frames: &mut Vec<protocol::AppFrame>, in_reply_to: Option<u64>, code: FaultCode, message: String) {
        push_app_fault(frames, in_reply_to, Fault::new(FaultOrigin::Os, code, message)).await;'''),
    (P, '''push_os_fault(&mut frames, Some(seq), "unsupported", ''', '''push_os_fault(&mut frames, Some(seq), FaultCode::new("unsupported"), ''', 2),
    (P, '''                *runtime.plugin_assembly_error.borrow_mut() = Some(Fault::new(FaultOrigin::Plugin, FaultCode::new(error.code), error.message));''', '''                *runtime.plugin_assembly_error.borrow_mut() = Some(Fault::new(FaultOrigin::Plugin, FaultCode::new("plugin.assembly-failed"), error.to_string()));'''),
])
