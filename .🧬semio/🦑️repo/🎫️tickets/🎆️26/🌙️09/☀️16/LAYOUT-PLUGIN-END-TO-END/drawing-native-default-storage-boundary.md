# Native Default and Storage Ownership Boundary

Actual owning attempts: --lib1 Source32 at tree data, ALL-target2 Source32 at old map tests, ALL-target3 Source32 at production Set (11447B versus7239bytes remaining, no file slots). No compiler or tests ran in those attempts. Public24 laws were retained in same-package targets; private production laws stayed with their owner. A defining runtime split is now required and approved by Root; no quotas change.

## Exact New Owners

- `semio-framework-value-native-storage`: `🧰️framework/🔨️modules/🌱️value/🛬️decode/🧳️owned/📦️storage/📦️packages/🦀️rust/Cargo.toml`, root wrapper there mounts the existing defining storage Rust file once. Canonical type `NativeOwnedStorage<T>` replaces private NativeReceiveStorage, with all actual callers hand-renamed; no compatibility alias.
- `semio-framework-value-native-default`: `🧰️framework/🔨️modules/🌱️value/🛬️decode/🧳️owned/📥️default/📦️packages/🦀️rust/Cargo.toml`, mounts existing actual trait/default cursor/slot/array/pair production files once. NativeOwned explicitly reexports these canonical public names. ALL standard and foreign first-party empty-owner impls of the local NativeDefault/NativeDefaultValue traits move with this trait, including OrderedMap/Set and PagedList. Domain-local Graph/Viewport/OS impls retain their actual local type owners.

Root registers actual projects/catalog/workspace rows and explicit unchanged262144/32/4000 caps once these paths/manifests are authored. No Source copies, duplicate mounts, test-only packages, runtime external dependencies or quota edits.

## Dependency Direction

Storage depends only on first-party Control/RetainedClone primitives and std allocation backend; no NativeOwned or Default. Default depends on Storage, NativeDecoding, RetainedClone/Retirement/Control, and actual PagedList/Ordered type owners. NativeOwned depends on Default and Storage. No lower owner imports the NativeReceive trait; it and ALL std/native external-type receiving impls stay together in NativeOwned to obey Rust orphan rules.

Default's current dependency on `super::tree` consists of local pure quote/receipt helpers and is removed; genuine helper definitions live with the default domain, with no authority construction. DefaultSlot has no NativeReceive dependency. Its actual output/close/CPU semantics and original finite profiles remain.

## Canonical Raw Typed Storage

`empty`, `reserve_demand`, `reserve`, `reserved`, `initialized`, `address`, `get/get_mut`, `move_demand`, `put_from(&mut Option<T>,grant)`, `take_into(&mut Option<T>,grant)`, `release_demand`, `release`, `take_box`, `terminal_is_empty` expose genuine allocation/custody.

Reserve preflight0, real success/null1, exact requested Layout bytes, null retains input and owner. `put_from` and `take_into` move existing caller-owned T with copy0/work1 at the actual pointer write/read; denied/cached0 and input/output slots retain custody. They execute no constructor, Default, FnOnce, predicate or payload destructor. `release` only accepts uninitialized backing and performs actual deallocation/work1; initialized backing refuses with0. Completed/cached release0. `take_box` moves actual initialized backing to Box without allocation/copy and work1.

Unsafe uninitialized-pointer borrowing is limited to the genuinely admitted non-whole-initialized allocation. RecordStorage retains its own initialized field counter and header-ready bit; its exact-field stores/removals remain defining funded operations. It releases only after actual live field count0. Generic storage never guesses whether a payload is empty.

Canonical receiver/default controllers create their documented allocation-free header only after their own full current preflight, keep it in a caller-owned Option, and `put_from` it in the admitted storage. Actual completed child C is moved by `take_into` into a retained header slot, that genuinely empty header is cleared in a distinct paid control turn, then raw backing is separately released. No blind close predicate/drop callback remains. Box output uses its existing pending T directly with put_from, no closure.

## Inline Prefix Reuse

The same storage owner defines `NativeOwnedPrefix<T,const N>` under `📦️storage/📋️prefix/🦀️.rs`: MaybeUninit fixed rows plus initialized prefix count. Pure header birth has no N callback/loop or heap. `push_from` admits one actual new row (sizeof T copy, work1), `pop_into` moves one retained last row (copy0, work1), and bounded existing-row swap moves ownership (copy0, work1). Borrowed prefix/last access is read-only and returns only initialized entries. No automatic body close or raw-array bypass; caller owns and funds popped T retirement. This is directly reusable by NativeIO PathBuf/Modified owners without Default or opaque initializers.

## Owning Evidence Required

Storage keeps its original actual null/denial/identity/Serde/raw release cases, adds genuine caller-slot conservation and inline-prefix0/3/4097-row tests with declared finite stock and independent Serde output. The old seven concrete/derived NativeReceive initializer law moves to the NativeOwned public target because it uses parent receiving types; it is not dropped or imported back into the lower production dependency.

Default keeps all four existing default laws, original data/cuts/profiles/maximum turns, and its allocating foreign Default compile-fail doc. They mount once at the actual new Default owning gate. NativeOwned keeps all remaining24 public/private/record_source/set laws with no duplication; count changes are reported per real owner, not called green. ALL-target NativeOwned plus unfiltered new Storage/Default owners and required docs remain mandatory. No acceptance or Source fit is asserted by this proposal.
