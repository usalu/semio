# Terrain Window Store admission

The ordinary native trace shows the original caller’s actual copy spend rising from 785,880 to 134,216,176 bytes. The remaining held stock was 1,183 bytes. The ARM64 Store constructor’s fixed copy term is 19,632 bytes before source and transaction charges. It fits the 32 KiB mounted preparation turn. The unquoted receiving scope can lend at most half of that turn before control charges.

The proposed patch runs the native-free Store constructor under the current mounted preparation grant, before opening a native scope. It keeps the wrapper’s original partition, actor, generation and revision checks. The same paid receipt field used for ordinary document, configuration and draft constructors carries the returned WindowConfig receipt. The mounted pump takes and posts it before another unit can run. An unfunded admission returns immediately with no receipt or native scope. Cancellation before admission still owns the original carrier. Cancellation after admission still delegates to the Store batch. Receipt refresh and ACK use their existing paths.

The outer pump currently counts its net receipt as progress for WindowConfig even if only receiving control posted. The patch includes WindowConfig in its existing Store-batch guard. It records the constructor’s paid receipt and later Store-returned ownership or checkpoints. A blocked native callback can still incur genuine control charges, while those charges alone do not advance the stall witness. The sparse witness prints the inner Store phase. An absent inner phase distinguishes the wrapper that has not acquired the batch.

The only production caller of the WindowConfig registry’s advance method is the mounted unit. Two stale two-argument calls in the tier-2 retirement law become explicit admission plus current native receiving. The stale-authority assertion moves to admission, where the former combined function performed it. That registered companion suite already uses an undefined `receiving_job` in seven other lane paths and passes obsolete argument shapes to other Store APIs. The patch leaves that pre-existing broader fixture work open.

After the Dev build has released its captured source and the patch is applied, run the following native checks:

- `repo --lang=en cargo --store=shared --configuration=prototype --parallelism=2 -- test --locked --offline -p semio-plugin-tests-13 --lib a_mounted_window_config_publication_resumes_its_refresh_and_queues_the_retained_receipt -- --test-threads=1` checks a paid native-free admission, the retained generation refresh, one WindowConfig page, and exact ACK and retirement.
- `repo --lang=en cargo --store=shared --configuration=prototype --parallelism=2 -- test --locked --offline -p semio-s-artifact-gis-gisterrain-editor --lib retained_commands_publish_only_their_declared_store_lanes -- --test-threads=1` checks the real `setCamera` publisher.
- `repo --lang=en cargo --store=shared --configuration=prototype --parallelism=2 -- test --locked --offline -p semio-s-artifact-gis-gisterrain-editor --lib gis_terrain_window_config_isolates_two_registered_windows_and_reloads -- --test-threads=1` checks the real second-window and reload path. The two Terrain filters can be grouped by running its library suite once.
- Once the unrelated fixture compile defects are resolved, `repo --lang=en cargo --store=shared --configuration=prototype --parallelism=2 -- test --locked --offline -p semio-plugin-tests-17 --lib every_publication_lane_retires_a_rejected_authority_without_faulting_each_turn -- --test-threads=1` checks the direct stale-authority path.

The ticket patch is prepared and passed a read-only applicability check against the live worktree. No production file was changed and no compiler was started for this preparation.

The checked patch is now applied. The focused mounted WindowConfig regression launch uses the current Prototype compiler entry. The standard GIS Dev retry includes the same Store changes.
