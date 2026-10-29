# Mesh Clipping R36

The fresh445independentanalysis-node oracle exposes22.838nm error at a nearly parallel material-cell intersection. Actualpublishednode[14.16,5.526557621,3.025] differs fromGEOS[14.16,5.526557598161949,3.025]. This exceeds thedeclared9decimalpublicationhalf-cell, not an acceptable newgolden orreason toincreasea10nmtolerance. Originalticket mesh-clip-r34.json/extractorretain sourceprovider4378442d…68a andtwosimple4vertex counterexamples fromSL-1parts194/195.

Cause: analyticaltranslate_structure repeatedlyrounds coordinates9decimals after everyhalf-planeclip. Near-parallel intersections amplifythatquantization beforefinalnodepublication. _half/_clip nowacceptdigits=None toretainfullintermediateprecision; ONLYtranslate_structure's material-cellclip optsintoit. Existing9decimaldefaultandexplicit12decimalphysicalmaterial/clippingsurfacecontracts remainunchanged. The existingfinalnodepublication remains9decimals.

Permanentlanguage-neutralmodel/tests/🕸️mesh-clipping.json/.py contains twocounterexamples andpublicationbound. Tworedtests passafterfix(.008s): independentGEOS domainxor<1e−12m², andbidirectionalSciPyKDTree publishednode distance≤sqrt(3)*.5e−9m+32floatingULP. No10nmfixtureallowance orprogressivetolerancebump. Thisnarrowfixwillbevalidated bytheexistingactualmainanalysis-method oracle, notonlyunittestedhelpercalls.

Changedfiles: model/analysisAPI andmodel/tests/🕸️mesh-clipping.json/.py. RootsourceJSONunchanged; new447staircandidatepending.
