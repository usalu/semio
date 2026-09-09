# Mounted Acknowledged Checkpoint Probe

## Contract

The direct browser actor publishes `browser-actor-ui-mounted` only after all of these owners agree:

- the exact current verified cold pair;
- the execution-target lease checkpoint and descriptor digest;
- the bound document port;
- the retained Shell UI revision; and
- the guest's completed acceptance of the Shell patch acknowledgement.

The closed mounted projection carries `activeCheckpointId`, `descriptorDigestV1`, and the exact five-field scope-bound frontier. The Hub two-peer harness requires both mounted peer projections to equal the canonical MCP checkpoint pair that it reads after the side-witness sockets observe the same `RebootstrapRequired.control`.

## Evidence

- Session `81290`: `cold-document-pair-browser-check` GREEN, 6 tests passed and 319 skipped.
- Language-neutral/third-party oracle: AJV accepted the source and projection, `fast-deep-equal` matched the independent projection, and 8 hostile source cuts were refused.
- Rebootstrap oracle: AJV 1, two independent peer reductions, 7 hostile source cuts.
- Log: `🗑️generated/mounted-checkpoint-probe/focused-cold-browser-current3.log`.

Sessions `29258` and `92443` were source-oracle RED before tests. The first exposed a stale Shell helper marker after browser actor retirement was centralized; the second exposed a hostile mutation that removed only one of two required render drives. Both predicates were repaired without weakening runtime admission.

## Nonclaims

The genuine two-author browser process has not been rerun on this source frontier. This evidence does not claim Chromium, WGPU, external model, or current native component acceptance.
