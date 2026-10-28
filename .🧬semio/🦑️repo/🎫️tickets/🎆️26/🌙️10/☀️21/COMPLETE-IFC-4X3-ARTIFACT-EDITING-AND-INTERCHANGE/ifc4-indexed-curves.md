# IFC4 Indexed Profile Curves

Schema-first contract at sharedgeometry/📜️indexed-curves.json and four neutral source graphs were authored before implementation. They failed for unsupported IFCINDEXEDPOLYCURVE, then passed after implementing native-value-reader typed metadata and controlled line/three-point arc sampling.

TypeScript supports IFC4 PointList2D source coordinates, IFCLINEINDEX2+ and IFCARCINDEX3 typed aggregates, original index continuity/closure, implicit coordinate closure, exactcollinear arc fallback to polyline, signed minor/major arcs through middle, and originalmiddle/endpoints. Each sourcepoint/index/sample consumes original caller work. Nativeport is a concurrent High executor scope and is not claimed executed.

Actual currentIFC4 geometrysuite39groups passes strictcompilation with zerodiagnostics. New differentialcase compares capsule extrusion volume with existing Three EllipseCurve/ExtrudeGeometry. Neutral majorclockwisearc, implicitline, collineararc cases and malformedconnectivity/closure/bounds/arity+cancellation are covered. A140000point implicitlist exhausts originalworkbudget with Limited ratherthan JavaScript argumentstack RangeError.

Official IFC4 slab-openings input is retained separately. It requires thiscurvefamily plus actual opening subtraction; currentengine stilldiagnoses and omits opening-dependenthosts. No fullEXPRESS/MVD or slabopeninggeometry success is implied.

## Native Port And Mapping Follow-Up

Native typed-value reader/interface and indexed line/arc helpers are implemented by High executor; Low static review found no concrete borrow/type defect. Native execution remains unverified. The original native fixture loop consumes the same four new graph vectors; separate native helper tests cover scale, CW/major intervals, collinear fallback and original cancellation.

A separate Low official-source mapping audit established required3D dimensions for mapping origins/axes and Axis2Placement3D; no cross-product recovery was substituted for singular normative omitted-axis defaults. Fiveof6new malformedplacement/mapping graphs failed before fixes. Both implementations now enforce exact3D point/direction arity, reject present nonreferenceoptional axes and reject derivedmarkers in optionalmapping Scale. CurrentTypeScript45groups passed with zero diagnostics before final strictoptionalaxis normalization (recheck pending).
