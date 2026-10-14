# Kernel Descriptor and Fault Taxonomy

The current TypeScript Kernel root still defines package descriptor admission, fault types, severity ordinals, and the fault error class alongside the runtime. The new descriptor text policy and package receipt identity have their own owners, but public descriptor admission still requires loading the entire runtime module. Kernel's package root also wildcard-publishes the monolith.

The next complete cut will move the actual descriptor admission definitions to `📇️descriptor/🟦️.ts` and the actual fault definitions to a neutral `⚠️fault/🟦️.ts` owner. Root publication will use explicit named exports and imports for the moved real bodies; no forwarding functions, duplicated definitions, copied cases, deprecated entry points, or Specific implementation dependencies will be introduced. Actual package facets for descriptor and fault will publish only the real named contracts. The severity and fault wire contract must be schema-owned and checked against independent Ajv while retaining original error payload identity and ordinal behavior.

The entire current six-law receipt/stream corpus and original Specific receiving command remain required. A language-neutral ownership vector will establish a genuine structural red before extraction, and descriptor/fault semantic receiving will execute the real moved definitions. New facets alone cannot certify the remaining wildcard runtime publication, all Rust counterparts, whole browser boot, or all Specific deletion.

This is a source-inspection and implementation plan, not a claim that the extraction or new facets already exist. The currently accepted descriptor and Go cuts are recorded separately in reports 55 and 58. No source bodies or cases are copied here.
