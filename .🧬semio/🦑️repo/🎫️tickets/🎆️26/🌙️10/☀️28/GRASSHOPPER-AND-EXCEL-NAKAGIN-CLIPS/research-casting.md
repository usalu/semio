# Grasshopper 1 implicit casting: authoritative table for the Nakagin clips

Scope: Grasshopper 1 (Rhino 7/8). Status: VERIFIED means the claim was read in a McNeel source (developer docs, API reference, forum post by McNeel staff or community) during this research. UNVERIFIED means no source found; the row must be tested in the running app before filming. Quotes are under 15 words. This file corrects the `[K]` rows of `research-grasshopper.md` section 2.2.

## 1. Mechanism and API facts

| Fact | Status | Source and quote |
|---|---|---|
| Cast order: derived type passes; else `CastTo<target>` on the source; else empty target `.CastFrom(source)`; both fail: error, slot blank | VERIFIED | https://developer.rhino3d.com/guides/grasshopper/grasshopper-data-types/ "If both these methods fail then Grasshopper will put up an error and leave that slot blank." |
| `GH_Conversion` has `Primary` (0), `Secondary` (1), `Both` (2) | VERIFIED | https://developer.rhino3d.com/api/grasshopper/html/T_Grasshopper_Kernel_GH_Conversion.htm Primary = direct casts between same/closely related types; Secondary = implied conversions excluding direct casts; Both = combined. |
| `GH_Convert` methods exist: `ToDouble` (not `ToNumber`), `ToInt32`, `ToBoolean`, `ToPoint3d`, `ToVector3d`, `ToPlane`, `ToInterval`, `ToCurve`, `ToBrep`, `ToString`; box only has `ToBox_Primary` | VERIFIED | https://developer.rhino3d.com/api/grasshopper/html/T_Grasshopper_Kernel_GH_Convert.htm "Convert data into Doubles." Each Primary is "direct cast from" the native or GH_ type. |
| `ToInt32_Secondary` is "indirect casts from other data types" (this is where Number to Integer lives; no rounding documented) | VERIFIED | https://developer.rhino3d.com/api/grasshopper/html/M_Grasshopper_Kernel_GH_Convert_ToInt32_Secondary.htm "Performs indirect casts from other data types into Integers." |
| `GH_Integer.CastFrom(object)` returns bool; the API page documents no behaviour | VERIFIED (absence) | https://developer.rhino3d.com/api/grasshopper/html/M_Grasshopper_Kernel_Types_GH_Integer_CastFrom.htm (no remarks). The same holds for the other `CastFrom` pages; no per-type rule table exists in McNeel docs. |
| `GH_Number` does not derive from `GH_Integer`, so Number to Integer is not a direct cast; Rutten recommends `GH_Convert` | VERIFIED (via search digest, thread not opened in full) | https://www.grasshopper3d.com/xn/detail/2985220:Topic:397835 "GH_Number does not derive from GH_Integer (or vice versa)" |
| Boolean casts to integer (false 0, true 1), string ("false"/"true"), colour | VERIFIED | Data types guide above: "false -> black, true -> white" |

## 2. Rule table (source type to target type)

| # | Source to target | Result, Nakagin example | Status | Evidence |
|---|---|---|---|---|
| 1 | Text to Number | Panel "2.3" to width Number gives 2.3 | UNVERIFIED rule detail (success case); failure case VERIFIED | Success implied by forum fixes that route Panel text through a Number parameter: https://discourse.mcneel.com/t/data-conversion-failed-from-text-to-number/136013 (answer: text box outputs text, put "a Num parameter" between). Decimal separator follows the locale: UNVERIFIED, test "2,3" on a German system. |
| 2 | Text to Number, fails | Panel "Kapsel" to width Number: red, slot blank | VERIFIED | Message: "Data conversion failed from Text to Number" (post lowercases it). https://discourse.mcneel.com/t/data-conversion-failed-from-text-to-number/136013 |
| 3 | Text to Integer, fails on empty entry | Panel with a trailing empty line to storey count: red | VERIFIED | https://discourse.mcneel.com/t/data-conversion-failed-from-text-to-integer/154859 Message "Data conversion failed from Text to Integer"; cause "`<empty>` is not an integer". |
| 4 | Number to Integer | Number 10.6 to storey count: rounds to nearest, ties to even (see section 3) | VERIFIED for ties, unresolved for non-ties vs truncation | Section 3. |
| 5 | Integer to Number | 11 to 11.0 (exact widening) | UNVERIFIED (no source read; trivially lossless) | Test in app. |
| 6 | Number to Boolean / Integer to Boolean | 0 to False, nonzero to True | UNVERIFIED | No McNeel source found. Test 0, 1, 2, -1, 0.5. |
| 7 | Boolean to Integer | False to 0, True to 1 | VERIFIED | Data types guide above. |
| 8 | Boolean to Text | "false" / "true" | VERIFIED | Data types guide above. |
| 9 | Text to Boolean ("True") | "True" to True | UNVERIFIED | No source found; test "True", "true", "1", "yes". |
| 10 | Point to Plane | Point (12.0, 3.5, 30.0) to Plane with origin there and world XY axes | VERIFIED (community, not McNeel staff) | https://www.grasshopper3d.com/forum/topics/plane-of-a-line and blog "shortcuts for grasshopper building blocks": point makes an XY plane with the point as its origin. Search digest quote: "grasshopper substitutes a default orientation: xy". Exact axis orientation UNVERIFIED; test in app. |
| 11 | Line to Plane | Line becomes the plane Z axis, origin at the line start | VERIFIED (community) | https://www.grasshopper3d.com/forum/topics/plane-of-a-line Oster: "interpreted as the Z (normal) vector of a plane" |
| 12 | Curve to Plane | Planar curve to its plane | UNVERIFIED | No source found. Test; likely only planar curves. |
| 13 | Integer or Number to Domain | Number 3.8 to Domain 0 To 3.8 | UNVERIFIED for Number; forum claim for Integer only | Forum reply (search digest): Integer into Domain gives 0 to that value. Page not opened. Test Number 3.8 and negative values. |
| 14 | Domain to Number | Domain 2 To 5 to Number | UNVERIFIED | One user observed domain length used, unconfirmed (https://discourse.mcneel.com/t/confused-about-construct-domain/205366, not opened). |
| 15 | Text "1,2,3" to Point | Point (1,2,3) | UNVERIFIED | No source found; searches surfaced none. Do not teach until tested. |
| 16 | Point to Vector | Point (x,y,z) to Vector (x,y,z) | VERIFIED (per existing quiz and Rutten on implicit conversion; Rhino API added implicit casts) | Rutten reports adding Vector3d/Point3d implicit conversions, search digest only. Existing repo quiz `cast-point-vector`. |
| 17 | Line to Vector | Line from A to B to vector B minus A | UNVERIFIED | Hypothesis from the Line-to-Plane thread ("your line is recognized as a vector"), which is a Kim statement, not an API fact. Test. |
| 18 | Point to Number | Distance from origin, not the Y coordinate | VERIFIED (community, search digest) | Digest: "returns the distance from the origin". Source thread not opened; confirm in app. |
| 19 | Circle to Curve | Passes as a derived type, no cast | VERIFIED by mechanism step 1 | Data types guide. In Grasshopper a Circle is a separate GH_Circle; whether it casts to GH_Curve via CastTo is UNVERIFIED. Test with a Circle into a Curve parameter. |
| 20 | Box to Brep | Box to Brep (six faces) | UNVERIFIED | `GH_Convert` has only `ToBox_Primary`, which suggests Box is a source type; `GH_Brep.CastFrom` accepting Box not read. Test. |
| 21 | Closed planar Curve to Surface / Brep | Curve to Surface fails; Curve to Brep UNVERIFIED | UNVERIFIED | `GH_Brep.CastFrom` may accept planar closed curves; not confirmed. Use Boundary Surfaces. Test. |
| 22 | Anything to Text | Number 3 to "3" | UNVERIFIED rule detail | `ToString_Secondary`: "calling the ToString() function" (https://developer.rhino3d.com/api/grasshopper/html/T_Grasshopper_Kernel_GH_Convert.htm). Number formatting (culture, digits) UNVERIFIED. |
| 23 | Point to Mesh, Text to Number "Wall" | Fails | Failure behaviour VERIFIED generically; per-pair UNVERIFIED | Data types guide: error, slot blank. |

## 3. Integer rounding

- Evidence for banker's rounding (ties to even), Integer parameter: David Rutten, 5 Jan 2010, reply to Scott Crawford, who said the rounding occurred in the "Int Parameter" (https://www.grasshopper3d.com/xn/detail/2985220:Comment:42581). He calls it the .NET standard: "the even number is returned"; 4.5 becomes 4, 5.5 becomes 6, so 2.5 becomes 2, 3.5 becomes 4.
- Conflicting evidence: Rutten, 30 May 2012, "converts a floating point number into an integer by chopping off the decimal part" (https://grasshopper3d.com/forum/topics/the-int-component). The context (an Int component with data from a Number vs Parameter dialog) was not reconcilable from the page, so the 2012 statement may describe a different path (parameter persistent data vs a wire). Note that "away from zero" is supported by neither source.
- Decision for the clips: state the tie rule as ties-to-even (2.5 to 2) only after filming a test of 2.5, 3.5, -2.5, 10.6 and 10.4 on Rhino 8 with a Panel into an Integer parameter. Teach Round, Floor or Ceiling components for deliberate rounding (Rutten recommends Floor and Ceiling in the 2010-era threads, e.g. https://www.grasshopper3d.com/xn/detail/2985220:Comment:995601, "The Ceiling function in the expression language is made for this.").

## 4. Failure message wording

- Pattern: "Data conversion failed from <Source> to <Target>". Verified instances: "Text to Number" (thread 136013, written in lowercase by the poster) and "Text to Integer" (thread 154859, exact capitalisation).
- Rutten (via digest) noted that newer versions insert nulls on a failed conversion in some components, a breaking change from older tutorials. Exact version is UNVERIFIED.
- Component turns red; the output slot is blank (data types guide).

## 5. To test in the running app before filming (priority order)

Rows 4 (ties and truncation), 6, 9, 12, 13, 14, 15, 17, 19, 20, 21, plus Panel "2,3" under the German locale and the exact capitalisation of the message on Rhino 8.

## 6. Limits of this research

The McNeel API pages carry no per-type CastFrom/CastTo remarks and no rounding spec. The grasshopper3d.com forum threads are archives of McNeel forum posts; rows marked "community" were stated by users, not McNeel staff. WebFetch summarises pages with a small model, so quotes were taken from its output rather than from raw HTML.
