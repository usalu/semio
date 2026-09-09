# Framework DAG Demo Owner

Host, Jack and Puzzle native builds all reproduced a missing include in the framework DAG artifact: its snapshot schema reached into the editor plugin's demo asset. The input moved during the fixture taxonomy correction, exposing the inverted dependency. The caller trace found framework DagHost::default_demo and numerous framework graph interaction/retirement tests depend on this generic five-node/four-edge demo. The correct common owner is the framework DAG artifact. The plugin is a consumer of that generic example.

A neutral node/edge identity fixture and native first-party JSON versus serde_json assertion were added before changing the source. Existing codec roundtrip and canonical DSL tests also cover the input. The authored DSL is being moved unchanged into the framework artifact's asset tree and exported by one named constant. Plugin example and IO entrypoints will use that constant; there will be no duplicate DSL asset and no framework-to-plugin read.

## Current Validation

The shared demo DSL bytes now live with the framework DAG artifact and all known framework/plugin consumers refer to its exported constant. The native neutral graph identity test was added before this source move. The first Nx attempt (`dag-demo-ownership-native-1.log`) stopped in unrelated concurrent taxonomy projection validation before Cargo. The second attempt uses the ticket's lazy domain-command router and is pending. The global field parity report attempt (`artifact-field-parity-note-10.log`) encountered the same taxonomy validation; no new parity count is claimed.

The earlier Note full native run (`note-document-contract-native-1.log`) reached SDK compilation and failed four incorrectly qualified empty-owner disposer references. They now use their canonical crate reexports. The Note tests themselves have not yet run.

## Native Verification

`dag-demo-ownership-native-2.log` completed through Bun/Nx with exit 0: all 57 framework DAG native tests passed, including the neutral demo graph identity test. Total runner duration 22m38s included waiting for the shared Cargo target.
