# BIM DDL44 Independent SQLite Audit

Actual light Bun SQLite execution read the mounted DDL. It created44 tables successfully. Width range2..55; maximum bim_stair55, roof32, opening29, material24, profile23, axis19, top13. Exact literal table ownership forecasts are in `📥️inputs/semantic-independent-oct8-bim-ddl44-expected-rich-counts.json`:44 tables/192 expected rows for the independently authored rich fixture. This is an authored expectation, not a provider output or Native/Source runtime receipt.

## Structural Checks Actually Executed

For bim_axis2 owner columns, bim_top4, bim_profile6, independent insertion with no owner failed, exactly one owner succeeded, two owners failed. These CHECKs use boolean IS NOT NULL sums and have no nullable equality bypass. Foreign keys were disabled for this isolated CHECK test, so no parent existence qualification is claimed from it. The actual DDL declares each owned relation's FK to the named structural parent, including baluster→railing, profile→six actual roles, hole_vertex→hole, property_set→property_element, property_value→set. Domain references remain *_key TEXT without existence FKs, correctly preserving unresolved references.

Custom and Explicit empty outlines retain the profile/space parent and kind; empty holes retain bim_slab_hole even without vertices; empty outer property element and inner property set have separate parents. Exact required shared-child counts for the rich witness are axis5, top11, profile17, baluster2. Profile17 = column types4 + beam types4 + curtain1 + railings rail/post6 + balusters2. Custom outline20 vertices is five Custom roles, independently from17 total profiles.

## Actual DDL Constraint Gaps, Reader Obligations

Independent SQLite admitted `INSERT INTO bim_profile(id,column_type_id,kind) VALUES(1,1,'Rectangle')` with width/depth bits/class NULL. It also admitted `INSERT INTO bim_property_value(id,set_id,ordinal,name,kind) VALUES(1,1,0,'p','Integer')` with integer_value NULL. The integer range CHECK evaluates UNKNOWN for NULL. Therefore actual DDL alone does not enforce active variant payloads. This is a demonstrated schema constraint gap; root's intended import reader validation can reject it, but that validation has not been executed here. No complete provider defect is inferred from a planned reader.

All required *_class columns currently accept arbitrary text, and nullable query values currently admit missing finite REAL. Optional triples permit mixed bits/class presence. Variant inactive columns are not constrained NULL. Existing enum discriminants and required boolean ranges are present; active/inactive values require exact structural reader checks or stronger static CHECKs.

## Narrow Supported Static CHECK Proposals

Use explicit non-null tests, not nullable equality, for active fields. Example property Integer branch: `CHECK(kind<>'Integer' OR integer_value IS NOT NULL)`, then exact inactive checks for all other scalar columns. Boolean similarly requires non-null and0/1; Text requires text_value IS NOT NULL. Floating property kinds require bits/class present and other scalar families NULL.

Required float triplet shape: class IS NOT NULL and within the authored class domain; bits IS NOT NULL; finite class requires query IS NOT NULL, nonfinite classes require query IS NULL. Optional float shape must be either all three NULL, or bits/class present plus corresponding finite/nonfinite query condition. Optional slope must make both direction and angle jointly absent/present. Native signed64 bits/class/query correspondence remains the reader's exact IEEE interpretation obligation; SQLite REAL does not preserve negative-zero sign.

Per variant, freeze exact payload slots. Axis Line requires all bulge cells NULL; Arc requires a present float triple. Top Unconnected requires height and forbids offset/storey_key; StoreyTop requires offset and forbids height/storey_key; Storey requires offset and storey_key and forbids height. Profile Rectangle width/depth; Circle diameter; IShape width/depth/web/flange; Custom no scalar profile payload. Roof Flat none; Shed pitch/direction; Gable pitch/ridge_direction; Hip pitch; Mansard lower/upper/break. Opening Window only window_type_key; Door only door_type_key; Void only width/height triples. Flight Straight none; LTurn split+turn in Left/Right; UTurn gap; Spiral radius/sweep. Infill None forbids thickness, Glass/Panel require it. Space Bounded seed_x/y, Explicit forbids seed triples.

Do not add UNIQUE because current framework parser refuses it. Duplicate per-role singleton children, map_key/ordinal collisions, noncanonical sorted maps, noncontiguous ordered children and orphan/unconsumed rows remain original import validations. SQL FK alone cannot require exactly one profile/axis/top child per owner, nor ensure outline only belongs to Custom/Explicit without cross-table checks. Audit the actual reader when mounted and run original independent negative cases there.

Source numeric bounds (panes u32, level/property.Integer i32) require the defining neutral schema decision documented in the totality report; DDL currently enforces those bounds. Ordinary positive widths/RGB/external-reference existence remain deliberately unconstrained.
