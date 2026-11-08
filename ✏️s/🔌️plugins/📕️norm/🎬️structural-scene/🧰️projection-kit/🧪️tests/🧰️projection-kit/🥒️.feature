@capability-projection-kit
@oracle-python
@comparison-closed-form
Feature: Projection kit shared by the Eurocode scene crates
  The vector, frame, binding and solid-anchor helpers must equal an independent Python implementation and obey the shared conventions.

  Further specification assertions, asserted by the Rust unit tests of the crate and not by the platform adapters:
  - Frames and yaw agree with the oracle: Given an axis along X, an axis along Z and a quarter-turn yaw When the member frames and the rotated direction are computed Then they equal the Python results
  - Boxes are anchored at the centre of their base: Given a box described by its lower corner, a frame and a size When it is converted to the schema convention Then the frame origin is the centre of the base and the corner is recoverable
  - Check statuses map to scene statuses: Given every status of a check report When it is mapped to a scene status Then an unverified basis is a warning and not applicable stays not applicable
  - Bindings follow the governing check: Given a report with several checks of one subject When the binding of that subject is built Then it carries the largest finite utilisation and ignores not applicable checks
  - Check stations become scene fields: Given a member check with four stations and a plate check with three stations When the per-vertex field, the per-segment colours and the point field are projected Then they equal the Python results and the binding carries ascending per-station values
  - An empty golden is blessed once and then guards the bytes: Given a golden file that is missing or empty When a scene is compared with it Then the bytes are written and the run fails once for review And the next run passes and any changed byte fails unless SEMIO_UPDATE_GOLDEN is set
