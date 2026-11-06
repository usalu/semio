# Owned Remodeling Rows Buffer

A product-private OwnedRowsBuffer takes actual owned Vec rows and grows capacity without copying an original source into a synthetic authority. Sufficient capacity transfers the existing header. Otherwise it reserves the required real capacity, moves one element per funded turn, reverses one pair per turn, and pays retirement of the old empty buffer before output transfer. Cancellation retains all input/output/retirement ownership. Constructor refusal returns the actual original input.

Six neutral order cases and strict schema were authored before the helper. Existing Lodash cloneDeep and an independent move-order check passed in normal repository Bun. Rustfmt passed. Native tests use the first-party backing allocation observer installed as the diff test binary allocator, compare actual birth/release with receipts, deny each physical axis and work, and close every successful frontier. Additional laws cover overflowing capacity and preserving sufficient original backing. Returned output is separately retired under grants; its final retirement was initially outside the backing observer and must not be described as a total end-to-end observed heap balance.

Native 50488 is live with no compiler/test verdict. The helper is not a completed keyed row or diff applier. Diff package entry/Cargo dev dependency explicitly owns the test allocator; no external runtime dependency was added.

Files: diff/🧳️retained/🧺️rows-buffer source and test; 🧫️owned-rows-buffer fixture/schema; diff Cargo/test entry; ticket 🧪️owned-rows-oracle.mjs.

