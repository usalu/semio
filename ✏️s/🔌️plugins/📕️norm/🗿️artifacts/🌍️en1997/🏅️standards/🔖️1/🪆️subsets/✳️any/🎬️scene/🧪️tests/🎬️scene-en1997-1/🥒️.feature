@capability-en1997-1-evaluate
@oracle-en1997-1-compliance-independent
@comparison-numerical-v1
Feature: EN 1997 structural scene
  The pure projection of the geotechnical snapshot and its check report must be deterministic, valid,
  incremental and numerically equal to independent numpy, scipy and shapely computations.

  Further specification assertions, asserted by the Rust unit tests of the crate and not by the platform adapters:
  - Ground model, footing, wall, slope and pile group are projected: Given the compliant geotechnical demo document and its settled report When the structural scene is projected Then the scene validates and holds one body per soil layer, the groundwater plane, the footing with effective area, the wall with pressure diagrams, the slope with slices and the pile group
  - Projection is deterministic and incremental: Given the compliant demo document When the footing width changes Then only footing owned entities change their content hash and the soil layers keep theirs
  - Bearing capacity factors and failure wedge: Given friction angles from 1 to 45 degrees and a strip footing of width 2.5 m with phi 32 degrees When the factors and the Prandtl mechanism are built Then Nq, Nc, Ngamma, the wedge areas, the spiral radius and the wedge depth equal the numpy and shapely results
  - Settlement stress under a rectangle: Given a 2.5 by 2.5 m footing with 100 kPa net pressure When the vertical stress is evaluated at several offsets and depths Then it equals the scipy double integral of the Boussinesq point load
  - Slip slices cover the sliding body: Given a 6 m high slope at 25 degrees and its critical circle When the slices are built Then their total area equals the shapely area of the circle below the ground line within one percent
  - Handles write the current field values at zero drag: Given the compliant demo document When every handle is evaluated at zero drag Then footing width, length, embedment and layer interface depths equal the document values
  - Diagrams carry table twins: Given the compliant demo document When the diagrams are built Then earth pressure, CPT, SPT, bearing factors and load-settlement diagrams validate and their twins have equal column lengths
  - Contact pressure stations integrate to the vertical action: Given the compliant demo document and its settled report When the soil-pressure stations of a footing with full base contact are integrated Then the integral over the base equals the design vertical action and equals the numpy trapezoid sum
  - Consolidation time track scrubs the settlement dish: Given the compliant demo document When the structural scene is projected Then the scene holds a time track whose keyframes name existing settlement fields and ends at the final settlement And the degree of consolidation at the time factor 0.197 is 0.5 as in Terzaghi's closed form
  - Check outcomes route to their entity: Given the compliant, the non-compliant, the showcase and a cased demo document When every check of the settled report is routed Then it names a row the scene draws (subject first, then the owner table of its family) or a document-level family that selects the global badge, and the entity carries the utilisation of its worst owned check
  - Every check family is drawn: Given the check families of the owner table When each is looked up Then exactly one of the dedicated diagrams or the outcome diagram of its entity draws it
  - Every numeric field has a grip: Given the snapshot schema When the numeric fields of each row collection are listed Then each has a geometric grip, a gauge writing it with twenty schema steps per metre, or a documented exemption, and a dragged value changes the projected scene
  - Characteristic bases are drawn and editable: Given a characteristic value basis of a layer When the scene is projected Then a disc stands in the layer with a parameter and a basis choice grip writing the typed tokens
