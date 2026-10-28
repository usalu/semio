# IFC Mapping Axis Conformance Audit

Read-only review of shared geometry operator at `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🏗️ifc/📐️geometry/🟦️.ts` lines187–196, against unchanged official IFC4 ADD2 TC1 EXPRESS retained as `official-IFC4-ADD2-TC1.exp`.

## Important correction to proposed repro

Axis3 parallel global Y with Axis2 omitted passes the dimensional entity WHERE rules, but the official derived-axis function itself produces a zero second projection. It is therefore not a sound case for asserting a unique expected conforming matrix. The current rejection of a zero direction follows this normative function's singular case. A cross-product fallback would be an explicitly chosen recovery interpretation of incomplete data, not a literal implementation of this release's EXPRESS. Keep diagnostics for explicit parallel axes rather than silently inventing their handedness.

For example `#1=IFCCARTESIANPOINT((0.,0.,0.)); #2=IFCDIRECTION((0.,1.,0.)); #3=IFCCARTESIANTRANSFORMATIONOPERATOR3D($,$,#1,$,#2);` gives z=Y, x=X, then y=Y−dot(Y,Y)Y−dot(Y,X)X=0 in IfcSecondProjAxis. Likewise omitted Axis1 with z=negative X projects the default positive X to zero; the official special case tests positive X only. These source inconsistencies must not be concealed as established normative defaults.

## Exact intended derivation

For 3D, normalize Axis3 or default it to positive Z. Derive x from Axis1 projected perpendicular to z; omitted Axis1 uses positive X, except exactly positive X z uses positive Y. Derive y by subtracting both z and x components from Axis2, defaulting that input to positive Y, then normalize. Order matters: Axis3, Axis1, Axis2. Axis1/Axis2 are hints, not independently normalized final columns. Explicit input Axis1 parallel z is undefined. A degenerate Axis2 projection also has no defined orientation. Do not force y=cross(z,x) for all inputs: it destroys legal reflection.

The columns of the transformation matrix are the derived orthogonal directions. Translation is LocalOrigin; uniform Scl is Scale or1. Nonuniform Scl2/Scl3 independently default to Scl. Every effective scale must be strictly positive; reflection comes from axis sense and determinant −1, not a negative scale. LocalOrigin and every present axis must retain dimension3 for the 3D operator. All optional attributes omitted gives the identity axes and unit scale, with the required LocalOrigin translation.

## Actionable implementation finding

`point()` at lines79–82 calls the shared padded coordinates reader; `operator()` uses it for LocalOrigin and all directions without checking their original aggregate cardinality. Thus a 2D point/direction is accepted and padded in a 3D operator despite DimIs3D/Axis1Is3D/Axis2Is3D/Axis3Is3D. Add operator-specific exact3D source validation, preserving normal 2D placement support elsewhere. Optional axis slots should distinguish unset from present malformed scalar/derived marker rather than treating any failed reference read as omission.

Invalid-source regression: `#1=IFCCARTESIANPOINT((4.,5.)); #2=IFCCARTESIANTRANSFORMATIONOPERATOR3D($,$,#1,$,$);` currently becomes translation(4,5,0); it must fail the 3D mapping dimension gate. An explicit `IFCDIRECTION((1.,0.))` in Axis1 must similarly fail.

## Fully defined valid neutral vectors

1. Identity defaults: origin(4,5,6), no axes/scale; input(1,2,3) becomes(5,7,9).
2. Reflection: origin(0,0,0), Axis1=X, Axis2=negativeY, Axis3=Z; input(1,2,3) becomes(1,−2,3), determinant−1. Preserve surface orientation handling.
3. Axis3 parallelY with supplied Axis2: Axis1=X, Axis2=negativeZ, Axis3=Y; input(1,2,3) becomes(1,3,−2), determinant+1. This is the defined counterpart of the singular omitted-Axis2 case.
4. Nonuniform version of case3 with Scale=2, Scale2 omitted, Scale3=4: effective factors(2,2,4); input(1,2,3) becomes(2,12,−4). With Scale omitted and Scale2=3, Scale3 omitted, factors(1,3,1).
5. Nonorthogonal hints: z=Z, Axis1=(1,1,1), Axis2=(0,1,1); x=(1,1,0)/sqrt2 and y=(−1,1,0)/sqrt2. Test projection rather than accepting raw direction columns.

## Primary sources

Pinned official release commit a8664407b63d42651e48a78a252c76d09dd6274c; schema SHA256 a3e46d39a85c2b683e7167572165d74b4ff6f8ef7e7c1e7f314a4980a63c9edb. Exact source links:

- [IfcBaseAxis](https://raw.githubusercontent.com/buildingSMART/Standards-builds/a8664407b63d42651e48a78a252c76d09dd6274c/IFC/RELEASE/IFC4/ADD2_TC1/HTML/schema/ifcgeometryresource/lexical/ifcbaseaxis.htm)
- [IfcFirstProjAxis](https://raw.githubusercontent.com/buildingSMART/Standards-builds/a8664407b63d42651e48a78a252c76d09dd6274c/IFC/RELEASE/IFC4/ADD2_TC1/HTML/schema/ifcgeometryresource/lexical/ifcfirstprojaxis.htm)
- [IfcSecondProjAxis](https://raw.githubusercontent.com/buildingSMART/Standards-builds/a8664407b63d42651e48a78a252c76d09dd6274c/IFC/RELEASE/IFC4/ADD2_TC1/HTML/schema/ifcgeometryresource/lexical/ifcsecondprojaxis.htm)
- [IfcCartesianTransformationOperator](https://raw.githubusercontent.com/buildingSMART/Standards-builds/a8664407b63d42651e48a78a252c76d09dd6274c/IFC/RELEASE/IFC4/ADD2_TC1/HTML/schema/ifcgeometryresource/lexical/ifccartesiantransformationoperator.htm)
- [IfcCartesianTransformationOperator3D](https://raw.githubusercontent.com/buildingSMART/Standards-builds/a8664407b63d42651e48a78a252c76d09dd6274c/IFC/RELEASE/IFC4/ADD2_TC1/HTML/schema/ifcgeometryresource/lexical/ifccartesiantransformationoperator3d.htm)
- [IfcCartesianTransformationOperator3DnonUniform](https://raw.githubusercontent.com/buildingSMART/Standards-builds/a8664407b63d42651e48a78a252c76d09dd6274c/IFC/RELEASE/IFC4/ADD2_TC1/HTML/schema/ifcgeometryresource/lexical/ifccartesiantransformationoperator3dnonuniform.htm)

No production changes or native execution performed.
