# Fleet owner pool keep-alive membership

The coordinator's read log records the first accepted 4,808-owner wave at line 2260. Subsequent offers carried 4,789, 4,786, 4,768, and 4,760 owners. At line 3056, a 4,768-owner offer refused the manifest phase after a CAS PUT received no answer by its 20,183 ms deadline. Several owners have been retrieved through the fleet path. The current log does not identify the caller for each offer or establish why the store did not answer.

The publisher reconstructed units and serialized a fresh session whenever the ten-minute keep-alive fired. A small reduction in open owners could therefore produce an entirely new units blob and wave ID even when the explicit shrink check had declined a publication. The loopback cache regression initially reported a 499-owner renewal of an accepted 500-owner wave and expected 500.

A keep-alive now renews the exact last accepted bulk manifest with a fresh head timestamp. Its original units, publisher, revision, eligibility metadata, and session bytes stay intact. Successful eligible shrink or settle replaces the bulk manifest. A refused candidate leaves the accepted manifest available for renewal, and a forced targeted ask uses its own candidate and tracking. Renewal still writes the manifest head and updates the pointer through the existing publication path. Blob existence checks let it reuse already-stored payloads.

The language-neutral JSON supplies the announce, keep-alive, shrink, settle, refusal, and targeted operations. A SQLite query independently selects the correct accepted membership for each renewal. The native test also checks the full head and units, refreshed pointer timestamp, stored session, and actual cache request sizes. After the change, all 10 keep-alive and early-wave tests passed and the patch passed the diff whitespace check. Broader owner-pool runs hit existing wall-clock performance assertions on this busy machine. Those assertions drive pullers directly and do not create a publisher.

No fleet publication or live process change was performed.
