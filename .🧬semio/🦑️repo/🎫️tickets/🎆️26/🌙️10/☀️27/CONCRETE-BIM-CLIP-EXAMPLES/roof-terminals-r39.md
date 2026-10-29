# Roof Terminal Retrieval R39

Read-only focused continuation,2026-10-29. Source model untouched. Read R36 and inspected original photo strips104 and original publication scans below. Parent owns lifecycle and generated cleanup.

## Successful Original Publication Retrieval

The useful new evidence is actual L’Architecture Vivante1931 primary-publication photographs, not another modern roof description. [BnF catalogue](https://gallica.bnf.fr/ark:/12148/cb327009292/date.item) identifies digitized1931 volumes but actual Gallica requests returned403 in both web and boundedNodefetch. [Cité authoritative exhibition](https://expositions-virtuelles.citedelarchitecture.fr/EXPO-VIRTUELLE-ARCHI-VIVANTE/08-PROJET-04.html) independently identifies the1931fourthseries containing MaisonSavoye and its original-author texts.

[Veredes archive index](https://veredes.es/blog/coleccion-completa-de-la-revista-larchitecture-vivante-1923-32/) links a publicZIP of scanned original journals; this is a third-party transfer path, not an assertion of archival chain-of-custody. The retrieved pages themselves visibly carry original publication imprint,1931date and plate numbers. Direct publicdownload ID1aERScJV8h0PBJOsrAE0lko7EYxSrDLaj. EntireZIP1,189,156,213bytes was not downloaded. HTTPRange65536-byte tail exposed centraldirectory. Retrieved only compressed1931.pdf entry at831,772,097..1,006,582,599 inclusive (174,810,503bytes including localheader), inflated to175,060,420bytes. SHA256: a80e104e935ff14b1e0b12cf17200cd3f93f0d76c34d5e7ec0f86a77554803cf. PDF under generated/vivante1931-r39.pdf. No PDF action,launch target or executable embedded content was run.

## Exact Page/Image Register

PDF bookmark Primavera-Verano(núms31–32)→Láminas→Pág27 resolves pageobject321. Resources1568generation1 points toJPEGimages1705/1706 plus scanned textmask1707. DirectJPEGstream extraction avoidsPDFJS/getOperatorList/worker decoding.

| Artifact under generated | Actual pixels inspected | Architectural content |
|---|---|---|
| vivante1931-plate27-image1705-r39.jpg |657×516| Top: southeast completed facade with tall prismatic projection right of the prominent solarium/stair body and slender projection near left end of that body. Bottom: same facade during construction. Both retain facade corners and pilotis for future projective controls. |
| vivante1931-plate27-image1706-r39.jpg |662×287| Separate view across two facades; cylindrical projection visibly rises immediately alongside stair body. Foliage obscures some facade/roof edges. |
| vivante1931-image320-r39.png |863×1063| Originalplate26: long facade, solarium/stair volume visible only in silhouette. Stored scan has90degree page rotation. |
| vivante1931-image335-r39.png |863×1063| Originalplate28: view up exterior ramp; tall cylindrical stack rises immediately beside stair-body wall on right, with flat dark open mouth and no ornamentalcap visible. Low rectangular projection at left near roof screen. Strong ramp-edge/door/wall correspondences are available; fullroofplan absent. |
| vivante1931-image340-r39.png |863×1063| Extracted adjacentplate29; not yet visually inspected, no claims based on it. |
| vivante1931-text27-r39.png |863×1063| Inspected and rejected: museum project, not Savoye. Distinguish textpage27 from photographplate27. |

The source photographs are substantially more usable than the twelve-thumbnail rita104 strip, but657px image1705 contains two pictures. It is not a highresolution measurement. The photographplate28 cylinder is a clear physical morphology confirmation; naming it the boiler requires pairing the p117function evidence and plan correspondence, not mere resemblance. Its open mouth supports excluding an invented ceramiccap for this photographed state. Exactdate correlation remains the author-book photo-register assignment; originalcaption itself does not label individual ducts.

## Registration Boundary and Parent Handoff

These actual photographs now permit photo-to-model comparison using facade corners, pilotis, ramp edges, stairdoor and stairbody. A singleperspective cannot by itself recover unseenroofXY. Do not turn imagepixel offsets into metre offsets using one facade width: chimney heights and varyingdepths displace the projections differently. No19572plan or measuredroofpenetration was recovered; consequently no publishableXYfootprint is claimed in this pass. The parent can overlay currentnewstairbody and solve camera controls against these sourcepixels, then combine an originalroofplan for depth.

Specific visible roof objects in1705top: prismatic rightprojection roughlyimagex503..518,y48..91; slender leftprojection roughlyx365..372,y43..74. Pixelboxes are retrieval/registration controls only, not elementidentities or metricdimensions.1706cylindricalprojection roughlyx401..411,y58..84.335 cylinder roughlyx128..344,y218..262 before90degreerotation. These smallboxes should be refined at originalresolution, not treated as subpixelmeasurements.

## Retrieval Failures

ExactFLC19572/canalisation2me queries yielded only officialUVabook captions, not originalplanpixels. No repeat52MBbookfetch. Officialp234–235 lowstacks/capcontinuation remains uncorroborated fromofficialindex inthispass; mirrorlead still not promoted. No newventblobs assigned. CCA known362311 is an underpilotis image fromearlierR35 and cannot registerroof. The newlyfoundZIP contains fulloriginal1931journal, not19572constructiondrawing; its scannedpublishedplans may be earlierdesign versions.

## Verification

VerifiedHTTP206contentrange,ZIPcentraldirectory entrysize, inflatedPDFheader and exactSHA256. Kept1931volume only in generated. ParsedPDFobjects without invokingPDFactions; resolvedbookmarks beforeextractingimages; checkedDCTJPEGheaders/dimensions throughview_image; decodedFlategrayscale adjacentpages withsystemzlib and a minimalPNGencoder. No runtimeproductdependency,sourceedit,testedit,AGENTSchange,Gitmutation orshellcommand.

## Original Roof Plan Phase Audit

Resolved and visually inspected originalTEXTp23 (groundplan) andTEXTp24 (firstfloorlargeplan plussmallroofplan), at generated/vivante1931-text23-r39.png andtext24-r39.png. Bookmarktextp24→action1036→page106→image110. PhotographPLATE26 isNOTthe roofplan. Textp24topright is the roofdiagram; allreferences must preserve this distinction.

The roofdiagram has planbounds approximatelyu477..829,v35..348 in863×1063pixels. Ramp is verticalnearimagex735; stairis nearx695,y251. Usingonlythewholepublishedroofrectangle as a provisionalaffinemapping tocurrentouterboundsX−.15..19.15,Y−.05..21.55 gives X=19.15−(v−35)*19.3/313 andY=21.55−(u−477)*21.6/352. Picturedstaircenter~695,251 mapsaboutX5.83,Y8.17 versusnewexecutedregistrationaxisX6.407145,Y9.18022. Evenallowingseveralpixels oftraceerror,this is notcoincident. The diagrams must not be silentlyalignedbytheirwholeenvelopes andtreatedassurvey. Roofdrawinguses anearlierarrangement oritsreproductiondistorts/cropscontrols; eitherway exactstackXYdoesnotfollow.

Therearesmallrectangles touchingcurvedscreenlimbs andseveraloutlinesnearstairs. The19443planned-columnwarning inofficialp119 applies: a squareattachedtoascreen is not automaticallyanexecutedchimney. The kitchenstackis specificallyomittedfrom19443 despitebuiltphotoproof. Thereforepublishedtextp24roofdiagram cannotvalidateallthreeexecutedterminals byitself.

Actualplate27photo1705topnonethelessclearlypreservestheexecutedprismaticrightprojection andslenderleftprojection. Plate28photo335preservesthecylindricalstack immediatelybythestairstraightwall andtheexteriorramp. Theseare usefulmorphology/relativeadjacency controls forparentcameraoverlay; originalFLC19572remains requiredto disambiguateplanfootprints/identities. No chimneyhas been assignedtothewrongdesignsquaretoreacha completionclaim.

Retainedinput roof-publication-r39.mjs was imported andexecuted forimage1706; re-extractedJPEG equalsoriginal34153bytes exactly. Thischecks extractionreproducibility,notmetricregistration.

## Two-View Cylinder Registration Handoff

At parent request, inspected every recovered original Savoye photograph on plates30–34,36–39. Plates31/32 show garden and living room;33 circulation;34 kitchen;36 hall;37/38 pilotis;39 gardener’s lodge. They add no visible roof-terminal base. Plate30 is valuable: it supplies a second azimuth of the cylinder seen close-up on plate28.

Two scan images were losslessly rotated90° clockwise, preserving every grayscale pixel: generated/vivante1931-plate28-upright-r39.png andplate30-upright-r39.png. Both are1063×863. Original unrotated images remain retained. This is a change of image coordinate convention, not photographic retouching.

Approximate hand-read image controls (origin upper-left, uncertainty roughly3–5pixels):

| Feature | Upright plate28 | Upright plate30 |
|---|---|---|
| Cylinder mouth center |818,130|756,274|
| Cylinder side positions | x797..838 | x748..766 |
| Lowest visible cylinder edge | about797,349, hidden by ramp guardwall | about756,386, hidden behind railing/wall |
| Curved stair-body roof silhouette | about692,218 through972,187 | about602,262 through689,273 |
| Far visible ramp-deck corners |554,430 and625,430|different side view; identify through source model |
| Near visible ramp-deck edge points |482,739 and766,739|different side view |

The two close views plus the exterior view inplate27image1706 permit a camera-registration attempt. Cylinder diameter40cm is independently supported by original-function evidence in official bookp117, rather than estimated from these pixels. Plate28 shows an open dark mouth with no added decorative cap. The actual base is occluded in both close views: the lowest visible point is NOT the slab penetration. Both views put the cylinder next to the stair body, consistently on the lower-X side of the ramp in current model orientation. Existing inferred trunkX4.7,Y9.4 therefore remains a plausible continuation hypothesis relative to stairaxis6.407145,9.18022; this is a hypothesis for projection testing, not newly measured XY.

Use the actual top-flight deck edges and new stair-body surfaces as camera controls. The full2.77m landing width must not be assigned to the visible single-flight deck without checking whichflight appears. RM-2 has landing boundariesX8.128..10.898,Y7.53..8.23, but the photograph shows one flight approaching it. The topological left/right relation is useful even before calibration. Root can solve two cameras against these structures and assess whether a vertical continuation at4.7,9.4 matches both cylinder silhouettes within a bounded error. The40cm cylinder diameter gives an independent image-scale check at its depth.

For the two square chimneys, plate27image1705top supplies clear narrow left and wider right silhouettes against the completed body. The lower construction picture is a different construction state and must not silently supply completed heights. Published roofplan rectangles may instead be designed unexecuted supports; no function is assigned by matching those rectangles alone. The supplied images support an explicitly inferred reconstruction only after camera/depth registration; no camera-fit numerical result has been fabricated in this read-only audit.
