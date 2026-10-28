# Calm Cursor Audit

## Result

Pass. Independent inspection of the final standalone preview found exactly one active Ansehen target, the root optimization topic, with no click events or opened topic/video panes.

## Evidence

Reviewed [motion contract](cursor-motion.json), [schema](cursor-motion.schema.json), [sampler](cursor-motion.py), and [renderer](render-calm-cursor.py). The timeline totals 11 seconds: 1.5 seconds idle, 2.5 seconds approach, 3 seconds stationary hover, 2.5 seconds return, and 1.5 seconds settle. The direct route runs from (1572,840) to (1554.4609375,751.3671875), staying in the bottom-right area.

Independently checked all 132 fresh native DOM observations at 12 fps in generated/calm-cursor-final/metadata.json. Native stage counts are 18/30/36/30/18. Every active layer chip rectangle was compared to the actual pointer position. The only hit across the batch was root/optimization; all 36 hover observations remained inside it. All pointer event arrays were empty, every pressed flag was false, every pane remained unopened, and opened_topic/opened_video/revealed_video stayed null. The only card revealed anywhere was root/optimization. Inactive nested chips are inert and do not count as active hover targets.

Delivery metadata contains 330 frame references at 30 fps, with 90 hover frames. Each reference matches its native observation and the renderer quantization mapping. These are repeated native captures, not 330 independent DOM measurements. The test agent independently validates FFmpeg resampling using a PyAV color oracle; see [test report](cursor-tests.md).

Visually inspected fresh native frames 0030 (approach), 0066 (stationary reveal), 0096 (return), and 0131 (settle). Motion remains local to the bottom-right optimization chip, the root card stays present, the preview reveals optimization content, and the final overview is restored. Shared camera behavior remains subtle; no hectic cross-screen navigation or second hover target appears. Endpoint stability reports exact source pixels. Root separately audits encoded movie timing and decoded boundary quality.

## Provenance and Scope

The initial 330-frame draft failed the shared lesson source-change handoff guard and was discarded. The audited final batch is a freshly compiled standalone DOM snapshot with canonical_overview=false; no shared lesson handoff cache certificate is claimed. Canonical two-level navigation remains unchanged. See [implementation report](cursor-implementation.md) for the recovery and retained authoring inputs. Temporary generated evidence is removed after publication, as required by the ticket workflow.
