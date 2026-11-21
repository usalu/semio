# Play staging and native capacity

The workspace has no staged browser prototype module directory and no published Play artifacts. The installed local browser path remains available. The older Play smoke results therefore refer to an earlier build.

The shared cache now answers and the fleet reports ten machines. Invoking the native component stage directly through repo got through workspace selection and failed before compilation in process.nativeBuild.image-host-free-insufficient. It required 6.1 GiB free on the host partition and measured 2.4 GiB. The mounted replica store's free capacity belongs to the replica data and is separate from the host partition used by the build image. Cargo's targeted native check has a separate 5.5 GiB minimum. Repository-owned dry runs for dependencies, graph and Rust found no immediately collectable entries, and CAS collection reported zero recoverable entries.

A later retry reached 5.32 GiB, below the 5.5 GiB Cargo minimum. The preview on port 6034 still serves its HTML shell. All five signed module receipt requests return HTTP 404. Port 6033 has no listener. The remaining linked components in the native Rust run cache lack the corresponding current browser publication.

The runtime checks still need a current build, DIN 4108 archive loading and document transfer, EN 1990 and EN 1992 UI capacity probes, and a fresh open for every app. The old bridge deadline and in-process GPU setup remain documented in the implementation ticket handoff.

A 5.7 GiB host-space retry was still below component admission. A separate DIN 4108 native scene check cleared its 5.5 GiB admission and remained in Bun build-tool setup. Starting a fresh Standards Play instance on port 6033 confirmed the DIN 4108 activation-lane receipt is absent. The older listener on 6034 remained available for a 15-route Chromium sweep.

The admitted DIN 4108 scene check later exited before Rust compilation. Native Cargo preparation waited 1,810 seconds for initial compiler headroom and reported 2.0 GiB free against a 4.0 GiB requirement. The host doctor independently reported the Rust image at 3% free and the host partition at 1% free. A fresh Rust area repository collector dry run identified zero removable entries.
