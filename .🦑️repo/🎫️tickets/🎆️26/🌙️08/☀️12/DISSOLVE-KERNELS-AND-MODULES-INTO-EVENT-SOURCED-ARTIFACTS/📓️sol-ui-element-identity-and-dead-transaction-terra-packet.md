# Terra Packet: UI Element Identity and Dead Transaction

## Objective

Relocate the qualified shared `{ id: string }` identity contract from the element collection to the specific UI-owner `element-identity` module, delete the unmounted transaction context, and remove its inert calls without changing each component's real callbacks or interactions.

## Preconditions and Baseline SHA-256

- HEAD recorded by coordinator: `5a1367dfcc90630c52dc2ec4de9526babe8d70f4`.
- `ElementProps`: `68687315b3fc355762903da3b83ac383411e75a2bf95dd068dac578acbb83091`.
- React barrel after released keybinding registrar: `de3c18afdb4a6cb03ef35814457c139547b268d7ba960748ff5bc4c652a52f99`.
- `ActionGroup`: `359a23469d634ca1ebdde1eb6023250962d2032b06ff69084485ee7d5e47bd7e`.
- `Ring`: `9e12b40a944ceb231bea40af1fe939104901da0881accb29f5ecc220c239901b`.
- `Textarea`: `02ea6b5ca0f26cd79fae1efe78f788e3e6fb020020d926bb01ba26cb5542c7e4`.
- `Stepper`: `aa2feeb6b9ae15be9d1da969985baa19c5c41be2341f2e0c5b4d182c34d51fc9`.
- `Slider`: `67bb360b9d963dfa7bcfb756e837ab6ecfef869ee17c66be0fea806117bbd08b`.
- `Select`: `8a8afea589e46653312097b7f142233ffb9e394a77da1cc72d87dff443ec61d3`.
- `Input`: `d63e24e058da709579dde5802fd81dcb56005c894d7925675f06f2b40447f858`.
- `HistoryTable`: `fa86480561c6ff3a9b3c5ebfe4e47d96605e692d164bcf3fd4b16ace6a556e8c`.
- `Tree`: `38d8d45eb7a1aecf282c6838f69ce1c90eb7496f56227860394b1ce42c58cf38`.
- `Toggle`: `b04ecacc64e06ebe3d4f7756600785eaad08a0c894189a2ab69e59b89d9461e2`.

Abort and report if any source hash differs, except that the coordinator will provide the released current barrel hash after the serialized keybinding gate.

## Writable Source Lease

- `🧰️framework/🔨️modules/🖱️ui/🧱️elements/🐹️ElementProps/🟦️component.tsx` (delete after rewrites)
- new `🧰️framework/🔨️modules/🖱️ui/🔨️modules/🆔️element-identity/🟦️component.ts`
- the eleven direct component files listed in the baseline
- one unique Terra acceptance Markdown in this ticket

The shared React barrel is coordinator-only. Stop after source rewrites and send a registrar handshake; do not edit the barrel.

## Exact Implementation

1. Create the concise region-structured module with the repository header and one repository-owned contract:

   ```ts
   export interface ElementProps {
     readonly id: string;
   }
   ```

2. Rewire `HistoryTable`, `Ring`, `Toggle`, `Textarea`, `Stepper`, `Slider`, `Tree`, `Select`, and `Input` directly to `../../🔨️modules/🆔️element-identity/🟦️component.ts` from their element directories.
3. Delete `Transaction`, `TransactionContext`, `TransactionProvider`, `useTransaction`, and `ElementBaseProps`. Delete the old component and empty directory; add no alias or forwarder.
4. Remove inert `useTransaction` imports, hook reads, dependency-array entries, and optional lifecycle calls from `Ring`, `Textarea`, `Stepper`, `Slider`, `Select`, and `Input`.
5. In `ActionGroup`, retain the explicit `startTransaction` and `finalizeTransaction` props and calls. Remove only the dead fallback context: use those explicit callbacks directly and leave all open-state semantics unchanged.
6. Do not alter the separate transaction callback contracts owned by other components.
7. Do not touch stories, CSS, package manifests, lockfiles, launch configuration, plugin sources, or generated census files.

## Coordinator Registrar

After the source handshake, the coordinator will replace the React barrel's old `Transaction`/provider/hook/base-props import/export with an explicit `ElementProps` type import/export from the new module. No dead transaction symbol remains exported.

## Required Static and Runtime Gates

After registrar signal:

1. active-source scan shows zero old path, `TransactionProvider`, `useTransaction`, `ElementBaseProps`, and context transaction lifecycle calls;
2. exactly the nine independent components and the barrel consume `ElementProps` from the new module;
3. `ActionGroup` still invokes only its explicit transaction callbacks;
4. scoped ordinary and cached `git diff --check` pass;
5. run once through Nx:
   - `bun nx run @semio-tech/ui-react:lint --skip-nx-cache`
   - `bun nx run @semio-tech/ui-react:typecheck --skip-nx-cache`
   - `bun nx run @semio-tech/ui-react:test-quick --skip-nx-cache`
   - `bun nx run @semio-tech/ui-react:build --skip-nx-cache`
6. record exact pass/fail classification and all final SHA-256 values in the acceptance Markdown; do not repair unrelated baseline failures.
