from specs.lib import *
HG = PL + '/🖥️host/🎭️guest-runtime'
CH = [
    ch('🧱', 'guest-types-and-runtime-trait', 'guest_types_and_runtime_trait', [(16, 211)],
       'Guest types and runtime trait: the package reference, the compiled handle, the guest instance with its state, the guest fault helpers and the `GuestRuntime` trait.'),
    ch('🤖', 'mock-guest-runtime', 'mock_guest_runtime', [(214, 593)],
       'Mock guest runtime: the scripted outcomes, the mock instance state and job step gate and the scripted mock guest runtime with its runtime implementation.'),
    ch('🧠', 'owned-runtime-adapter', 'owned_runtime_adapter', [(601, 673)],
       'Owned runtime adapter: the compiled and instance accessors of the owned interpreter runtime, the component snapshot transport and the guest runtime implementation for it.'),
    ch('🏠', 'wasmtime-host-state-and-wit-hosts', 'wasmtime_host_state_and_wit_hosts', [(710, 1026)],
       'Wasmtime host state and wit hosts: the actor host state, the wasi context, the store fuel and the trap classification and every wit host interface the actor state implements.'),
    ch('🐎', 'wasmtime-runtime', 'wasmtime_runtime', [(1029, 1267)],
       'Wasmtime runtime: the wasmtime instance state and the wasmtime runtime with its compile, instantiate and sqlite observation entry points.'),
    ch('🗂', 'wasmtime-guest-runtime-and-codec-dispatch', 'wasmtime_guest_runtime_and_codec_dispatch', [(1270, 1635)],
       'Wasmtime guest runtime and codec dispatch: the guest codec dispatch over every runtime and the guest runtime implementation for the wasmtime runtime.'),
    ch('🎛', 'guest-runtimes-and-test-doubles', 'guest_runtimes_and_test_doubles', [(1638, 1847)],
       'Guest runtimes and test doubles: the guest runtimes enum with its runtime implementation and conversions and the recording runtime test double.'),
]
SPEC = dict(orig='hg.rs', file=HG + '/🦀️.rs', children=CH, wiring='block', external=mounted(HG + '/🦀️.rs', orig='hg.rs'))
