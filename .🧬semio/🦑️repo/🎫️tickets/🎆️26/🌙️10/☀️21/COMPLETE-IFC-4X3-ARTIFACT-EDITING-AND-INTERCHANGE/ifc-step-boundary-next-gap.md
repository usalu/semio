# Next IFC STEP Boundary Gap

Read-only actualsourceprobes. IFC2x3 own IO TSdecoder accepts mixedFILE_SCHEMA(('IFC2X3','IFC4')) andduplicateIFC2X3 schemaentries. Exactprobe minimalframedfile has#1=IFCAXIS2PLACEMENT3D(#999,$,$); allmixed/duplicate/singleIFC2X3 casesacceptedwithoneentity,includingdangling#999. Thisisnot4graphconversion; ownfacadevalidationonlytestsanynestedIFC2X3string.

Nativebase 🚪️io/🦀️.rs decode_ifc2x3 uses same anynestedlistcontainsIFC2X3predicate afterparse_part21, noownreferencepass. Encode calls validate_ifc2x3_snapshot, givingasymmetricimport/exportadmissionpossibility. Nativebehaviorisstatictrace, notexecuted. IFC4TS boundaryalreadyrequires exactlyoneschemaaggregatewithoneIFC4name andpositiveuniqueu64 IDs pluscompleteforwardreferenceexistence; own2 should get equivalent ownstandardexchangeadmissionpolicy withoutrequiringfullEXPRESS.

Focusedroot-ownednextslice: sharedversion-neutral boundedexchangegraph/headercheck behindownversionadapters, or own2local strictframing/header/schema/referencevalidator reusedbyTS/native decode+encode. Preserve exactPart21Decimal andEDMpreamble handling, do notcastIFC4snapshot. Addneutralcases mixed/duplicate/malformedFILE_SCHEMA,ID0/duplicate/maxu64,forwardvalidreference,danglingnestedtyped/listreference,badheaderarity andcancellation. UseexistingsharedPart21parser asparsingauthority; actualsnapshotIDs/references/header validationisartifactboundarywork. Firsttests shouldfailontheactualpublicdecodeIfc2x3facade.

Candidatefiles: own2base 🚪️io/🟦️.ts validate/decode/encode; own2base 🚪️io/🦀️.rs decode_ifc2x3; own2snapshotvalidator; own2🚪️io/🧪️tests andsharedlanguage-neutralexchangefixture. ThisdoesnotoverlapHighDetailsinputreceiverorHighgeometrypageorchestration. No newnativebuild/systemchangesneeded. Existingfullcatalog/WHERE/MVDvalidator isnotrequiredforthissmallboundarycontract.

Probeusedcurrentown2source+existingJSONawareticketloader,11underlyinggeometrygroupspassed0diag. Ownruntransformedmodulesdeleted; loaderinputretained. No newfixturesdownloadedorproductionchanges.
