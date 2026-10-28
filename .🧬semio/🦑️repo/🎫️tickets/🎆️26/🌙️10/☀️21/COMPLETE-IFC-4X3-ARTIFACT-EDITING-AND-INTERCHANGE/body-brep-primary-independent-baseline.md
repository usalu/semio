# Brep Primary Policy and Original Baseline

The pinned released Body Brep HTML at commit610ee3f95db9d0380b1eb8de4dc6e17833531518 lists Body/Brep/IfcFacetedBrep at local lines121–123. This is an explicit selected three-predicate projection. The adjacent AdvancedBrep schema family must not broaden it. ADD2 IfcShapeRepresentationTypes Brep branch13620–13623 tests FacetedBrep ancestry; AdvancedBrep13625–13628 separately accepts ManifoldSolidBrep. FacetedBrepWithVoids is included by inheritance; advanced breps and abstract manifold ancestry alone are insufficient.

Fresh independent IfcOpenShell0.9.0.post1 declaration metadata agrees with all18 neutral input expectations. Three unmodified original files were parsed and fully EXPRESS-validated behind the existing repository-owned oracle interface:

| Original representation | Selected Brep | Actual-type function | Brep function on actual items | Full EXPRESS statements |
| --- | --- | --- | --- | ---: |
| brep-model.ifc1020 | pass | true | true | 0 |
| basin-faceted-brep.ifc1155 | fail | true | true | 0 |
| cube-advanced-brep.ifc183 | fail | true | false | 0 |

The basin's actual SolidModel label makes it a selected-clause negative despite a faceted item; cube's AdvancedBrep label/item is a basic-Brep negative. These are original observations, not retagged examples. Exact header identifiers, SHA256, physical item IDs/classes/ancestry and raw diagnostic arrays are retained in `🗑️generated/brep-independent-oracle.json`; input brep-independent-oracle.py reuses the external backend only behind the owned interface. No new external dependency/install was introduced.

This establishes metadata/schema baseline only. Face-loop topology, shell closure/manifoldness, geometry and view applicability are not inferred from the selected predicate or external empty diagnostics. Implementation freeze/source audit remains separate. Required consumer laws remain original private request/source capture, exact declared-owner query and late unknown, actual original callback/signal authority, exact diagnostic pointers, immutable policy and reversible central mutation/STEP paths. All audit processes exited and no production file was edited.
