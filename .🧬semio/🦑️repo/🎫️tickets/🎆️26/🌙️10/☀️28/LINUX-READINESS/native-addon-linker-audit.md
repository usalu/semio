# Native Addon Linker Interfaces

Read-only authoritative Source/API inventory after the real rustc child refused missing cc. No compiler, Native, setup, probing or Source edits were invoked.

Process/tools registry JSON id cc declares compile/link on Linux/macOS/Windows and provision repo setup native. Its ordered declared locators are SEMIO_TOOL_CC, registered cc/gcc/clang discovery and installed native-compiler SDK. resolveRegisteredToolV1 returns an owned ToolResolution with command,prefix and optional suffix; missing command must refuse. routeRegisteredCommandV1 is unsuitable as a missing-linker authority check because its ordinary funnel returns the original command unchanged when no resolution exists.

The SDK adapter process/tools/native-compiler exports nativeCompilerArgumentsV1 and nativeCompilerShimSourceV1. Its GCC suffix includes explicit binutils search, system library search and SDK include coordinates; selecting only resolution.command loses this required materialization. A direct rustc -C linker command therefore must preserve the registered linker driver’s complete arguments through an existing owned driver or an explicitly proved argument projection. Do not rediscover ambient cc independently or silently retry PATH.

Existing durable materialization owner: caching/storage/enforcement/compiler/native-compiler/TS withRepositoryNativeCompilerV1(storage,control,environment,body). It requires a live enforcing compiler reservation, hashes bounded SDK+runtime closure, atomically writes a read-only content-addressed cc driver under that reservation, and rechecks driver and complete closure. repositoryNativeCompilerConfigurationV1 exposes its exact identity/environment to Cargo configuration capture. Its environment projection respects explicit compiler/linker flags, host/target distinctions and user configuration. This owner is compiler-store/Cargo-specific, so do not call it outside its lifetime simply to solve a lazy addon import.

Cross-platform probing is separately modeled by runtime/native/toolchain/TS and its schema: macOS developer-directory and Windows Visual Studio linker discovery are explicit ports. The current cc tool registry names cc/gcc/clang; it does not itself model MSVC link.exe as the same driver. Preserve existing rustc MSVC selection/arguments and registered OS-specific behavior; a Linux GCC suffix is not portable to that path.

Resolution/materialization must remain inside the actual existing addon build invocation, never module import, and carry its Original/control/deadline and admitted tools. The audit does not authorize wider compiler grants or claim that a successful tool selection conveys storage authority. Specific API findings were sent directly to the implementing Bootstrap owner.

## Frozen Narrow Projection Review

Reviewed actual frozen service birth mapper, Node addon implementation and Linker protocol. No material blocker found. The service resolves registered cc only on Linux, within its existing Original/store/Nx tooling use lifetime, and passes the same usePhase to its callback. The projection accepts only native-compiler source, rejects nonempty prefix or null command, and preserves existing explicit provider/addon/linker choices. Ordinary system compiler and nonLinux/MSVC selection are not rewritten.

Node runtime consumes only closed version1 DATA, exact three keys, absolute command at most256UTF16 units, at most64 nonempty arguments each at most256units, and no ASCII control characters. The canonical decoded DATA participates in the addon cache identity. Compiler arguments use explicit -C linker and one -C link-arg per registered suffix, preserving argument boundaries without PATH rewrite, shim creation or recursive Native storage admission. Registry access occurs at service birth, not runtime import; the addon exposes no registry type dependency.

The implementing owner reports genuine3tests34assertions green and full semantic17roots4303files0. This reviewer launched no tests or Native work and approved only this Source boundary for the coordinator-authorized single retry. Actual daemon acceptance is a separate gate.
