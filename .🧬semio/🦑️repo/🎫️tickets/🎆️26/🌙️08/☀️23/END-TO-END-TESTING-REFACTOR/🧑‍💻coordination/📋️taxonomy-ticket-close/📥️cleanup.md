# Reviewed Ticket Output Cleanup

Retain authored scripts, configs, fixture inputs, code preimages and every Markdown report. Preserve authored prose found in old text reports as Markdown. Remove only enumerated tool output after a hash check; the ticket generated directory is removed once all workers finish.

```json
{
  "rename": [
    {
      "from": "📊️verification-w12-animate-sequence-shooting-flow.txt",
      "to": "📊️verification-w12-animate-sequence-shooting-flow.md",
      "sha256": "9de8a9d29a78528f1e64daffd9318dc5b6e98dd600b896b8154dcb415bb6a762"
    },
    {
      "from": "📊️verification-wave2.txt",
      "to": "📊️verification-wave2.md",
      "sha256": "cf1cf0bef905b028563aef2c9ad5e77de35dfd63499cd78f708534c76d5dac01"
    },
    {
      "from": "📊️verification-wave3.txt",
      "to": "📊️verification-wave3.md",
      "sha256": "1b936d29dc272156f26afdfdaa45d9e948a41f3e4611fca765b70c8ad9ae2040"
    },
    {
      "from": "📊️verification-wave4.txt",
      "to": "📊️verification-wave4.md",
      "sha256": "1fc8ea6e5100beeac6bf141a5caf3eceee75ec3ccc8d996dd2ba74d066a54201"
    },
    {
      "from": "📊️verification-wave5.txt",
      "to": "📊️verification-wave5.md",
      "sha256": "50fc7376ea5633762398cd038c1c2c6db840256a0c4e65191a2fe98fa8a38864"
    },
    {
      "from": "📊️verification-wave6.txt",
      "to": "📊️verification-wave6.md",
      "sha256": "87331067ec9cf09d457d0469bc82249e849569d8c8251f35ae463770962e95b4"
    },
    {
      "from": "📊️verification.txt",
      "to": "📊️verification.md",
      "sha256": "d65a4d31bc3f96c57a6a939e1719a3f74a4752c10ca48e809a63ccb878786afd"
    },
    {
      "from": "w15-audit/no-oracle-rationales.txt",
      "to": "w15-audit/no-oracle-rationales.md",
      "sha256": "a8605cc0970f989a364aa52fe5f8a47fa222b91d7a9b61c172d9746cf35ae755"
    },
    {
      "from": "🧪️w14-semio-av-cad-doc-cross-language.txt",
      "to": "🧪️w14-semio-av-cad-doc-cross-language.md",
      "sha256": "adb0fb210ee755c9564e5384cd5168fa4d3369d410d23de82b50b23ae41c6e83"
    }
  ],
  "remove": [
    {
      "path": "🧪️w13-html5-parity-4.txt",
      "sha256": "c95b226c19759615917aeb3efa09fd3811df6f4bbb5fd49b173c2d1cabbdfd38",
      "bytes": 86,
      "reason": "Reviewed historical command/search output"
    },
    {
      "path": "🧪️w13-html5-parity-5.txt",
      "sha256": "c95b226c19759615917aeb3efa09fd3811df6f4bbb5fd49b173c2d1cabbdfd38",
      "bytes": 86,
      "reason": "Reviewed historical command/search output"
    },
    {
      "path": "w19-crosscutting/34-oskernel-sync-final-detail.txt",
      "sha256": "662f45ffc5be6e9ff448a816a02a17c6860b35dbbb4ce35087106de31a9bfdbd",
      "bytes": 27725,
      "reason": "Reviewed historical command/search output"
    },
    {
      "path": "w19-crosscutting/12-shared-after-quintet.txt",
      "sha256": "c6eaa1b09125556c666c69a0a52d74c9141da108e1929a4b8a192b53685c86d1",
      "bytes": 65463,
      "reason": "Reviewed historical command/search output"
    },
    {
      "path": "w19-crosscutting/20-oskernel-sync-pass7.txt",
      "sha256": "76c4ee8645ac7b6cba882702231633a609cc77f0e9b3153c01ad9c1d506ba991",
      "bytes": 12707,
      "reason": "Reviewed historical command/search output"
    },
    {
      "path": "w19-crosscutting/05-pass4.txt",
      "sha256": "287aa88cccfc3807a09fd431506f06dc6d0b8ad4c2ee0f8a70ddf8a46c1ba3d3",
      "bytes": 116826,
      "reason": "Reviewed historical command/search output"
    },
    {
      "path": "w19-crosscutting/03-pass2.txt",
      "sha256": "5f007b410d03c6298327670733f370ac1335bd1c5930242c4796dce180acb470",
      "bytes": 112932,
      "reason": "Reviewed historical command/search output"
    },
    {
      "path": "w19-crosscutting/24-oskernel-sync-pass10.txt",
      "sha256": "855fc1fdbb958db88b1f6c390c40d38c2d5f02915151083a978a91bb1a112a92",
      "bytes": 14616,
      "reason": "Reviewed historical command/search output"
    },
    {
      "path": "w19-crosscutting/15-oskernel-sync-pass3.txt",
      "sha256": "85630ee728fc37812b0c061e58896fb869af2cb652fe86a161f47176d3d649f0",
      "bytes": 12432,
      "reason": "Reviewed historical command/search output"
    },
    {
      "path": "w19-crosscutting/10-oskernel-sync.txt",
      "sha256": "8d29e63552a962e8100db5dc1d036928a751bdf2ed5395edb853cbaa10315ae3",
      "bytes": 41251,
      "reason": "Reviewed historical command/search output"
    },
    {
      "path": "w19-crosscutting/51-prewarm.txt",
      "sha256": "da3961b1d918514332b16a11dc051d5d9421e591118380af10704ee5ee80ed9f",
      "bytes": 1608,
      "reason": "Reviewed historical command/search output"
    },
    {
      "path": "w19-crosscutting/06-stdio-lib-test.txt",
      "sha256": "c6facca6af5b89b2e12333143416f61e4da575b2af9801be2315d87f28f547a7",
      "bytes": 480588,
      "reason": "Reviewed historical command/search output"
    },
    {
      "path": "w19-crosscutting/50-parity-obj.txt",
      "sha256": "257723f3ba1239e501875ff5bdba34c978dc6a7c8d2aa4e8e73b6495268d72f1",
      "bytes": 2831,
      "reason": "Reviewed historical command/search output"
    },
    {
      "path": "w19-crosscutting/42-deflate-one.txt",
      "sha256": "a7edbce7ef5c7a4aaab250cd726abcd0dcfb37778acf21e20d618237e1a85bb7",
      "bytes": 235365,
      "reason": "Reviewed historical command/search output"
    },
    {
      "path": "w19-crosscutting/52-prewarm-subject.txt",
      "sha256": "4df1377b592397a30bd288742d3677a07b69556843d21e2215231e723013ca64",
      "bytes": 109615,
      "reason": "Reviewed historical command/search output"
    },
    {
      "path": "w19-crosscutting/await-sites.txt",
      "sha256": "d15cbb8b4bdf97858a57d46767e102d4ac8d9bc681f13e4340f89665ae04e0b4",
      "bytes": 7399,
      "reason": "Reviewed historical command/search output"
    },
    {
      "path": "w19-crosscutting/builtnode-tests.txt",
      "sha256": "929ee6c9b14c095abbe17f73dd5d336d51853472372ade39893cc545505fe88f",
      "bytes": 54273,
      "reason": "Reviewed historical command/search output"
    },
    {
      "path": "w19-crosscutting/32-oskernel-default2.txt",
      "sha256": "7b50b0888c5d98b7adcdd9a6d544f771779efa46d57bb6f2893b41f91ebaacaa",
      "bytes": 7130,
      "reason": "Reviewed historical command/search output"
    },
    {
      "path": "w19-crosscutting/25-oskernel-sync-pass11.txt",
      "sha256": "dfee7b158dd0c454418cac9189d503929131bdb4fc94ac4a4bd823736e09921d",
      "bytes": 11671,
      "reason": "Reviewed historical command/search output"
    },
    {
      "path": "w19-crosscutting/04-pass3.txt",
      "sha256": "ba280585be955d13397c3f91887594ad8b6bbddc56519051bde18537ee9f2d95",
      "bytes": 97179,
      "reason": "Reviewed historical command/search output"
    },
    {
      "path": "w19-crosscutting/builtnode-files.txt",
      "sha256": "60cd05d293906a20ede7ee5fe8d16e9f758aed861a6b6e3ab09df7a2b2a544c5",
      "bytes": 7727,
      "reason": "Reviewed historical command/search output"
    },
    {
      "path": "w19-crosscutting/01-stdio-test-check.txt",
      "sha256": "139350f4dc14ec037027de4aa73998b6b8858729d88e02f7063dae24e389edbd",
      "bytes": 157278,
      "reason": "Reviewed historical command/search output"
    },
    {
      "path": "w19-crosscutting/16-oskernel-sync-pass4.txt",
      "sha256": "e257b74d8cf4548a96dc497cec9e2b2894d0ff441bd9a6b15ba6a827eddcbcc3",
      "bytes": 11737,
      "reason": "Reviewed historical command/search output"
    },
    {
      "path": "w19-crosscutting/errs2.txt",
      "sha256": "e644a3ea4916b2bb0c52933410d3aaa32b6390d96efba0cbc36cec00504d0a75",
      "bytes": 17013,
      "reason": "Reviewed historical command/search output"
    },
    {
      "path": "w19-crosscutting/02-shared-sentences-before.txt",
      "sha256": "9b41f7f32351064389d1d42e6700f4479dc45524e87fb268caf772c9668e2786",
      "bytes": 75454,
      "reason": "Reviewed historical command/search output"
    },
    {
      "path": "w19-crosscutting/19-oskernel-sync-pass6.txt",
      "sha256": "1bd2f194a5220b7eb852f9b770f761c727ab273677f27172652aaf5174261020",
      "bytes": 17854,
      "reason": "Reviewed historical command/search output"
    },
    {
      "path": "w19-crosscutting/22-detail.txt",
      "sha256": "71a8f8eab8ef961bdce08b4b9807936d17465d3bf95dd599f5d9dda734f0d647",
      "bytes": 27695,
      "reason": "Reviewed historical command/search output"
    },
    {
      "path": "w19-crosscutting/21-oskernel-sync-pass8.txt",
      "sha256": "e8eb976a874d7d95009a138cab4d5b3ed4158823dad4e3ded2aa9f50f80a8daa",
      "bytes": 11671,
      "reason": "Reviewed historical command/search output"
    },
    {
      "path": "w19-crosscutting/31-stdio-lib.txt",
      "sha256": "91636fbda50865fedf85c93aa019d282d24835becfc9a6b807d0b0e8b92616b3",
      "bytes": 11545,
      "reason": "Reviewed historical command/search output"
    },
    {
      "path": "w19-crosscutting/61-contract-final.txt",
      "sha256": "802222476b743b4bc04ed80ad28f54e8e11165c871008abb6956a1845cde4964",
      "bytes": 459,
      "reason": "Reviewed historical command/search output"
    },
    {
      "path": "w19-crosscutting/14-contract.txt",
      "sha256": "802222476b743b4bc04ed80ad28f54e8e11165c871008abb6956a1845cde4964",
      "bytes": 459,
      "reason": "Reviewed historical command/search output"
    },
    {
      "path": "w19-crosscutting/18-oskernel-sync-full.txt",
      "sha256": "88740a7213640824077e627b00045adcf75c98bd40221aafaef0806d7688a1d5",
      "bytes": 31239,
      "reason": "Reviewed historical command/search output"
    },
    {
      "path": "w19-crosscutting/40-stdio-testcheck-final.txt",
      "sha256": "6566fba75636509c520ee872c43ca66aa760d829ccd62e87a040bdd5e9521a72",
      "bytes": 69630,
      "reason": "Reviewed historical command/search output"
    },
    {
      "path": "w19-crosscutting/23-oskernel-sync-pass9.txt",
      "sha256": "74474099e3d41469ab4498d879201d6a52b11e48083388fd92aab6f0845f076a",
      "bytes": 11988,
      "reason": "Reviewed historical command/search output"
    },
    {
      "path": "w19-crosscutting/33-oskernel-sync-final.txt",
      "sha256": "b3bdc18e4ac4923f112b2e9823e6b22e45d9733679b5d3b37ccca996f935b7f4",
      "bytes": 11683,
      "reason": "Reviewed historical command/search output"
    },
    {
      "path": "w19-crosscutting/11-shared-after-norm.txt",
      "sha256": "7966b050159f2e95b1addf38aa2eeabac4e44722b232a021a7422f9b8c5a10ee",
      "bytes": 67858,
      "reason": "Reviewed historical command/search output"
    },
    {
      "path": "w19-crosscutting/errs.txt",
      "sha256": "53077cd5239a25f1643c35606d686ff1bb17cd8ee5114230bfef7855c34cbac7",
      "bytes": 59260,
      "reason": "Reviewed historical command/search output"
    },
    {
      "path": "w19-crosscutting/13-oskernel-sync-pass2.txt",
      "sha256": "5aba83dea0f93e090e860d7c96d12a82e27b951d8b0e0c8ff3fc064c0def253d",
      "bytes": 13385,
      "reason": "Reviewed historical command/search output"
    },
    {
      "path": "w19-crosscutting/60-shared-final.txt",
      "sha256": "c6eaa1b09125556c666c69a0a52d74c9141da108e1929a4b8a192b53685c86d1",
      "bytes": 65463,
      "reason": "Reviewed historical command/search output"
    },
    {
      "path": "w19-crosscutting/17-oskernel-sync-pass5.txt",
      "sha256": "1ac936cfeba0988fb65d6a12cdd58625d4a57733aa3c9ff43b5d1d18aff790a8",
      "bytes": 11737,
      "reason": "Reviewed historical command/search output"
    },
    {
      "path": "w19-crosscutting/26-detail2.txt",
      "sha256": "f71260a3917e147de893675b040c9becc8a28ae5bfc82a6118fd0244bb21917e",
      "bytes": 27651,
      "reason": "Reviewed historical command/search output"
    },
    {
      "path": "w19-crosscutting/43-deflate-two.txt",
      "sha256": "a0e18f445bb4c34579f4dc31316b92b26c7db47bc198557bd6b2e65b6fa34d4d",
      "bytes": 235628,
      "reason": "Reviewed historical command/search output"
    },
    {
      "path": "w19-crosscutting/30-oskernel-default.txt",
      "sha256": "57d32f56c2fd7e5ca61768d4d6e2773cad39efe2b2e094c3071bbbab0ddb366d",
      "bytes": 7387,
      "reason": "Reviewed historical command/search output"
    },
    {
      "path": "w19-crosscutting/unused-imports.txt",
      "sha256": "c0bfeadfbf278be722e9f53aa7fd5d900b59dbe640ab52307c02d3e230788dac",
      "bytes": 4180,
      "reason": "Reviewed historical command/search output"
    },
    {
      "path": "w13-carrier-probe/📄️probe-output.txt",
      "sha256": "d907c9dfcdb27c0c9e9d6884ad86bd0415f7918da12ffa7619731add1e0f9981",
      "bytes": 180508,
      "reason": "Reviewed historical command/search output"
    },
    {
      "path": "🧪️w13-html5-parity-2.txt",
      "sha256": "6761b6ba711c636e3f6cbebe12d0c3e7057bf95b0075073430e1b3ec14374cac",
      "bytes": 1319,
      "reason": "Reviewed historical command/search output"
    },
    {
      "path": "📓️object-subject-exhaustive-output-2026-08-24.txt",
      "sha256": "ffadad87689dc4cccd71345f3f805c926650bf4a29e647a9e057b22303daf332",
      "bytes": 83103,
      "reason": "Reviewed historical command/search output"
    },
    {
      "path": "w16-semio-drawing-mesh-image-presentation/🧪️w18-reverification.txt",
      "sha256": "49b43a35d80b09376421d5c7e6f8ceabcf4be180ffb190c650ea2a4392cd6d31",
      "bytes": 2393,
      "reason": "Reviewed historical command/search output"
    },
    {
      "path": "w14-ooxml-cad-parity/📊️after.txt",
      "sha256": "1340d70e3a02dee43c9ecf6eed5cef2359b6a427dbea1de2ae013862cc9ba0f9",
      "bytes": 2345,
      "reason": "Reviewed historical command/search output"
    },
    {
      "path": "w14-ooxml-cad-parity/📊️results.txt",
      "sha256": "c1769b852cb2f7d960e0c684697c09a37c506d12c51d1c98889fbe59e94abf09",
      "bytes": 1598,
      "reason": "Reviewed historical command/search output"
    },
    {
      "path": "w14-ooxml-cad-parity/📊️baseline.txt",
      "sha256": "045b80da65cb47661789ebebb9915a5c94bf1d523859c32324d4dda3ce7761eb",
      "bytes": 906819,
      "reason": "Reviewed historical command/search output"
    },
    {
      "path": "🧪️w13-html5-parity-3.txt",
      "sha256": "f329db7673bff6209af80384279fb1fd667f9af304aa562a224a8866b89c5f75",
      "bytes": 240,
      "reason": "Reviewed historical command/search output"
    },
    {
      "path": "🧪️w13-html5-parity-1.txt",
      "sha256": "43ace4a9167b41b10a66ec70c9605432b7df764a0042002a1d227c93cd62ab97",
      "bytes": 178445,
      "reason": "Reviewed historical command/search output"
    },
    {
      "path": "w15-work/00-baseline-norm-oracle.txt",
      "sha256": "a016a6d25670e53f8e439e1b292940b497dabe3685e16e76f4acae58cf4f786d",
      "bytes": 94,
      "reason": "Reviewed historical command/search output"
    },
    {
      "path": "w15-work/02-contract.txt",
      "sha256": "ef324681ff9c40df806c3d1572dfec189c61c433f266f391d4cc360f0f7e26a5",
      "bytes": 459,
      "reason": "Reviewed historical command/search output"
    },
    {
      "path": "w15-work/nondiff.json",
      "sha256": "39d3a1862ada4727ccf22f02e58843568b30b98b803e7ff38c4d72269b0bce66",
      "bytes": 87070,
      "reason": "Historical generated audit inventory"
    },
    {
      "path": "w15-work/nondiff.err",
      "sha256": "e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855",
      "bytes": 0,
      "reason": "Tool result stream or stderr"
    },
    {
      "path": "w15-work/03-dependency.txt",
      "sha256": "3735b63467e35ab4dd54251429d0973cd88ddc81f4abc19b2e4fa0bb8f3771d5",
      "bytes": 4234,
      "reason": "Reviewed historical command/search output"
    },
    {
      "path": "w15-work/nondiff-after.json",
      "sha256": "adccff3be473fcf021ed4cd233d037c8bd6e58a5ca7622f1e3aaec066ee6b8ab",
      "bytes": 78522,
      "reason": "Historical generated audit inventory"
    },
    {
      "path": "w15-work/01-after-shared-module-norm-oracle.txt",
      "sha256": "a016a6d25670e53f8e439e1b292940b497dabe3685e16e76f4acae58cf4f786d",
      "bytes": 94,
      "reason": "Reviewed historical command/search output"
    },
    {
      "path": "w16-structural-carriers/🧪️w16-kit-parity.txt",
      "sha256": "0577e1238ee0071b7d345cfa49b1be01791b76da31305b60a6b32edfe9c48c02",
      "bytes": 86,
      "reason": "Reviewed historical command/search output"
    },
    {
      "path": "w16-structural-carriers/🧪️w16-par-object.txt",
      "sha256": "1b4d890682ca41621f213d11a4e976dce080a0e75d179efd175a171e71324651",
      "bytes": 93,
      "reason": "Reviewed historical command/search output"
    },
    {
      "path": "w16-structural-carriers/🧪️w16-or-object.txt",
      "sha256": "04beeff3a89aeda3d3c6fd97df3254e145d54500c6f72ededc7f745b04972cd7",
      "bytes": 84,
      "reason": "Reviewed historical command/search output"
    },
    {
      "path": "w16-structural-carriers/🧪️w16-par-graph.txt",
      "sha256": "25de7a44be561220ed880bec45c1920035a2408a258d66cb28de82e61085a91d",
      "bytes": 93,
      "reason": "Reviewed historical command/search output"
    },
    {
      "path": "w16-structural-carriers/🧪️w16-dep2.txt",
      "sha256": "91e5a92e6418570d3daa6b202b5018b8d95e70c686ad315a922847a3b1b993d0",
      "bytes": 4101,
      "reason": "Reviewed historical command/search output"
    },
    {
      "path": "w16-structural-carriers/🧪️neg-object.txt",
      "sha256": "9b776bdb9ec1a7065fb3cb5032a0dd9f5bae9c376154f8a0f5685007d0d46a3b",
      "bytes": 84,
      "reason": "Reviewed historical command/search output"
    },
    {
      "path": "w16-structural-carriers/🧪️w16-or-graph.txt",
      "sha256": "307287b0a507e0d7497e0870936afdc6d21af3371f5881fcd02c9889dc921b8d",
      "bytes": 84,
      "reason": "Reviewed historical command/search output"
    },
    {
      "path": "w16-structural-carriers/🧪️w16-or-brep.txt",
      "sha256": "33488a2c1d58b9047af079dc3307709444eccde128df8b6462eafa9f9c4fad0d",
      "bytes": 84,
      "reason": "Reviewed historical command/search output"
    },
    {
      "path": "w16-structural-carriers/🧪️w16-contract2.txt",
      "sha256": "802222476b743b4bc04ed80ad28f54e8e11165c871008abb6956a1845cde4964",
      "bytes": 459,
      "reason": "Reviewed historical command/search output"
    },
    {
      "path": "w16-structural-carriers/🧪️w16-brep-parity.txt",
      "sha256": "157051224187437aaee7d2650d0483be9c7f3296a09b9fceabad5fc102ff7b73",
      "bytes": 86,
      "reason": "Reviewed historical command/search output"
    },
    {
      "path": "w16-structural-carriers/🧪️w16-or-kit.txt",
      "sha256": "dc71d5ee6502471e91aa85d0a7a691e4af818a054c83ec2b10ef40712ffba0f3",
      "bytes": 84,
      "reason": "Reviewed historical command/search output"
    },
    {
      "path": "🧪️w13-txt-subject-2.txt",
      "sha256": "ca698bb5358ec3f21d50000ce3dd2c80e7131879487d2e3d6c9f1d757df6edac",
      "bytes": 84,
      "reason": "Reviewed historical command/search output"
    },
    {
      "path": "zip-2-0-mutate/contract-output.txt",
      "sha256": "911bdcc3fc46bd487555ec764b64d31fb4f91e396bb60e8bde065ba4f6cc53fc",
      "bytes": 1107,
      "reason": "Reviewed historical command/search output"
    },
    {
      "path": "zip-2-0-mutate/oracle-exhaustive-output.txt",
      "sha256": "ae288ee2488376837ab665b3d4701c01b3a526fa9e467151850c0c01fe5a5bf1",
      "bytes": 84,
      "reason": "Reviewed historical command/search output"
    },
    {
      "path": "zip-2-0-mutate/poll-log.txt",
      "sha256": "1f04198b8986e22c4490ba6f6584d5f062a5a4976a50af2faa3d147c32c4d4ba",
      "bytes": 49,
      "reason": "Reviewed historical command/search output"
    },
    {
      "path": "w20-semio-tabular-verify/dependency-after.txt",
      "sha256": "91e5a92e6418570d3daa6b202b5018b8d95e70c686ad315a922847a3b1b993d0",
      "bytes": 4101,
      "reason": "Reviewed historical command/search output"
    },
    {
      "path": "w20-semio-tabular-verify/owner-oracle-after.txt",
      "sha256": "a681239ed9f2b477dbc1b2576efdd1ffd5965e099b82218fb97d55442196fb76",
      "bytes": 1744,
      "reason": "Reviewed historical command/search output"
    },
    {
      "path": "w20-semio-tabular-verify/parity-mutate-semio-text.txt",
      "sha256": "af4dedf37d48090fe5d22545c92fa2a6b4f36c395435fd6c3d27e114c5f974db",
      "bytes": 46313,
      "reason": "Reviewed historical command/search output"
    },
    {
      "path": "w20-semio-tabular-verify/regress-parity-mutate-semio-text.txt",
      "sha256": "f5b3a451ba9273320dbe4a7c6c83d0d8deacea3f47f63a74b7c133d51cfa2036",
      "bytes": 86,
      "reason": "Reviewed historical command/search output"
    },
    {
      "path": "w20-semio-tabular-verify/attempt1-mutate-semio-model.txt",
      "sha256": "aa8ba1b28b1c9fad8b6020486bacca1e8ffabc1c1660fb82186ff1df555ca2ec",
      "bytes": 29044,
      "reason": "Reviewed historical command/search output"
    },
    {
      "path": "w20-semio-tabular-verify/fixed2-parity-model.txt",
      "sha256": "83830bf46a539d96ba60313961becba8e095c05b10e0620339a5b90ec65d879b",
      "bytes": 86,
      "reason": "Reviewed historical command/search output"
    },
    {
      "path": "w20-semio-tabular-verify/final3-parity-model.txt",
      "sha256": "83830bf46a539d96ba60313961becba8e095c05b10e0620339a5b90ec65d879b",
      "bytes": 86,
      "reason": "Reviewed historical command/search output"
    },
    {
      "path": "w20-semio-tabular-verify/parity-mutate-semio-flow.txt",
      "sha256": "157051224187437aaee7d2650d0483be9c7f3296a09b9fceabad5fc102ff7b73",
      "bytes": 86,
      "reason": "Reviewed historical command/search output"
    },
    {
      "path": "w20-semio-tabular-verify/attempt4-mutate-semio-text.txt",
      "sha256": "e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855",
      "bytes": 0,
      "reason": "Reviewed historical command/search output"
    },
    {
      "path": "w20-semio-tabular-verify/regress-parity-mutate-semio-flow.txt",
      "sha256": "157051224187437aaee7d2650d0483be9c7f3296a09b9fceabad5fc102ff7b73",
      "bytes": 86,
      "reason": "Reviewed historical command/search output"
    },
    {
      "path": "w20-semio-tabular-verify/final-parity-mutate-semio-model.txt",
      "sha256": "e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855",
      "bytes": 0,
      "reason": "Reviewed historical command/search output"
    },
    {
      "path": "w20-semio-tabular-verify/warm5-mutate-semio-flow.txt",
      "sha256": "6e13a7014351c4cba44525dccf4391e85eb274a93ec129272e5e5dad2ef0906f",
      "bytes": 200614,
      "reason": "Reviewed historical command/search output"
    },
    {
      "path": "w20-semio-tabular-verify/attempt2-mutate-semio-model.txt",
      "sha256": "e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855",
      "bytes": 0,
      "reason": "Reviewed historical command/search output"
    },
    {
      "path": "w20-semio-tabular-verify/parity-mutate-semio-value.txt",
      "sha256": "f99210980e09988313c34b03be15aa34d76c977e926c7b6827e98345729bd51f",
      "bytes": 86,
      "reason": "Reviewed historical command/search output"
    },
    {
      "path": "w20-semio-tabular-verify/warm5-mutate-semio-text.txt",
      "sha256": "647c46e59a12836d6027c5187d57cc94bba0de2bd15935115a4affbe82bbf9a7",
      "bytes": 199,
      "reason": "Reviewed historical command/search output"
    },
    {
      "path": "w20-semio-tabular-verify/retry1-parity-mutate-semio-model.txt",
      "sha256": "0f3ba7f9750bebe31cb9b6c18171047b4fd96d0cb10be4be6a7e2df4ba9883ca",
      "bytes": 188919,
      "reason": "Reviewed historical command/search output"
    },
    {
      "path": "w20-semio-tabular-verify/parity-mutate-semio-model.txt",
      "sha256": "3877bafe33ea9c1a8bfb04584368d884c39f9733f6137d94a69f8f825ed6e9ef",
      "bytes": 2903,
      "reason": "Reviewed historical command/search output"
    },
    {
      "path": "w20-semio-tabular-verify/warm5-mutate-semio-value.txt",
      "sha256": "9fd1e14e8172a32a335050c896f2595bf5c9d39c8591e682a1cab983a4fe9b50",
      "bytes": 200615,
      "reason": "Reviewed historical command/search output"
    },
    {
      "path": "w20-semio-tabular-verify/regress-parity-mutate-semio-value.txt",
      "sha256": "bda74c2687d7889061dba756915419bd08251368f71d638162d03324d2acf12d",
      "bytes": 189092,
      "reason": "Reviewed historical command/search output"
    },
    {
      "path": "w20-semio-tabular-verify/negctl-mutate-semio-table.txt",
      "sha256": "307fc9251ce60f43eb884e41c066060cd32bef21d3ffbe49fa1c3e86dab1d00e",
      "bytes": 84,
      "reason": "Reviewed historical command/search output"
    },
    {
      "path": "w20-semio-tabular-verify/probe-parity-model.txt",
      "sha256": "3258afa94615b8165ff8f7d70b2a7eca47ad2728493fe962457d6f6793ffe18d",
      "bytes": 408,
      "reason": "Reviewed historical command/search output"
    },
    {
      "path": "w20-semio-tabular-verify/attempt4-mutate-semio-model.txt",
      "sha256": "dee117e131bba443b973b84db264a460efca654de1492a3f7ca60b3c5bb548fa",
      "bytes": 2904,
      "reason": "Reviewed historical command/search output"
    },
    {
      "path": "w20-semio-tabular-verify/final2-parity-model.txt",
      "sha256": "e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855",
      "bytes": 0,
      "reason": "Reviewed historical command/search output"
    },
    {
      "path": "w20-semio-tabular-verify/attempt3-mutate-semio-model.txt",
      "sha256": "3877bafe33ea9c1a8bfb04584368d884c39f9733f6137d94a69f8f825ed6e9ef",
      "bytes": 2903,
      "reason": "Reviewed historical command/search output"
    },
    {
      "path": "w20-semio-tabular-verify/warm2-text.txt",
      "sha256": "d61dfafa4c22f0dfd68e17cbe36f957fdf9da7ab28095593c6ac08e8b8a80e78",
      "bytes": 200614,
      "reason": "Reviewed historical command/search output"
    },
    {
      "path": "w20-semio-tabular-verify/postrevert-mutate-semio-model.txt",
      "sha256": "307287b0a507e0d7497e0870936afdc6d21af3371f5881fcd02c9889dc921b8d",
      "bytes": 84,
      "reason": "Reviewed historical command/search output"
    },
    {
      "path": "w20-semio-tabular-verify/postrevert-mutate-semio-value.txt",
      "sha256": "36c01f016e5507749b760f7db164c45d1a57940c2bbfafba3bbf298186174844",
      "bytes": 84,
      "reason": "Reviewed historical command/search output"
    },
    {
      "path": "w20-semio-tabular-verify/final3-parity-text.txt",
      "sha256": "f5b3a451ba9273320dbe4a7c6c83d0d8deacea3f47f63a74b7c133d51cfa2036",
      "bytes": 86,
      "reason": "Reviewed historical command/search output"
    },
    {
      "path": "w20-semio-tabular-verify/oracle-mutate-semio-flow.txt",
      "sha256": "33488a2c1d58b9047af079dc3307709444eccde128df8b6462eafa9f9c4fad0d",
      "bytes": 84,
      "reason": "Reviewed historical command/search output"
    },
    {
      "path": "w20-semio-tabular-verify/fixed-parity-model.txt",
      "sha256": "64d2b3b3a3c11979a21d80f5a4dcf798696ddf3243896c65fd7321b0496618f3",
      "bytes": 407,
      "reason": "Reviewed historical command/search output"
    },
    {
      "path": "w20-semio-tabular-verify/warm2-model.txt",
      "sha256": "470efcf04d956ec0f3c0ad9475c750b3ea7730dc1139faabd8d1e160d04ca1ba",
      "bytes": 188481,
      "reason": "Reviewed historical command/search output"
    },
    {
      "path": "w20-semio-tabular-verify/oracle-mutate-semio-text.txt",
      "sha256": "19236ada7e728d8135d75aa3c8ee63548417636b540071b45479fd35220a4dd7",
      "bytes": 84,
      "reason": "Reviewed historical command/search output"
    },
    {
      "path": "w20-semio-tabular-verify/contract.txt",
      "sha256": "802222476b743b4bc04ed80ad28f54e8e11165c871008abb6956a1845cde4964",
      "bytes": 459,
      "reason": "Reviewed historical command/search output"
    },
    {
      "path": "w20-semio-tabular-verify/negctl-mutate-semio-value.txt",
      "sha256": "676cc9258c4dc57a0f5bf5d53b5526c80456b10297f95c01356a2f8cfc82a8b0",
      "bytes": 84,
      "reason": "Reviewed historical command/search output"
    },
    {
      "path": "w20-semio-tabular-verify/regress-parity-mutate-semio-table.txt",
      "sha256": "077eac328a3f0e66bf4aa1c8c9cff6f43cb828f1788ecbbfe553d0b986905692",
      "bytes": 86,
      "reason": "Reviewed historical command/search output"
    },
    {
      "path": "w20-semio-tabular-verify/warm5-mutate-semio-table.txt",
      "sha256": "7fb699bf3f7c08f5220645bc250b603c122ac41a1c6386b3f4f39eaff9292666",
      "bytes": 200616,
      "reason": "Reviewed historical command/search output"
    },
    {
      "path": "w20-semio-tabular-verify/regress2-parity-value.txt",
      "sha256": "f99210980e09988313c34b03be15aa34d76c977e926c7b6827e98345729bd51f",
      "bytes": 86,
      "reason": "Reviewed historical command/search output"
    },
    {
      "path": "w20-semio-tabular-verify/retry1-parity-mutate-semio-text.txt",
      "sha256": "81d043ad71c3116003ee05c161a51b400eef5c22c33c027eb158a002819ad4eb",
      "bytes": 48979,
      "reason": "Reviewed historical command/search output"
    },
    {
      "path": "w20-semio-tabular-verify/parity-mutate-semio-table.txt",
      "sha256": "077eac328a3f0e66bf4aa1c8c9cff6f43cb828f1788ecbbfe553d0b986905692",
      "bytes": 86,
      "reason": "Reviewed historical command/search output"
    },
    {
      "path": "w20-semio-tabular-verify/warm4-model.txt",
      "sha256": "205153a102cf8a72be58c36d0f70217681156aaf63fd30d48e04221b8b84035f",
      "bytes": 187903,
      "reason": "Reviewed historical command/search output"
    },
    {
      "path": "w20-semio-tabular-verify/oracle-table.txt",
      "sha256": "1ac216d1bda0deac167892dc3afcb3df3db039a17c541ba223f2e2dc49040f92",
      "bytes": 84,
      "reason": "Reviewed historical command/search output"
    },
    {
      "path": "w20-semio-tabular-verify/contract-after.txt",
      "sha256": "802222476b743b4bc04ed80ad28f54e8e11165c871008abb6956a1845cde4964",
      "bytes": 459,
      "reason": "Reviewed historical command/search output"
    },
    {
      "path": "w20-semio-tabular-verify/warm3-model.txt",
      "sha256": "53d82053f818a7463c2edf5e5770cb459aadfaf0fd0dc62f5c001917f22456b0",
      "bytes": 188481,
      "reason": "Reviewed historical command/search output"
    },
    {
      "path": "w20-semio-tabular-verify/retry2-parity-mutate-semio-model.txt",
      "sha256": "012fe4ed8815efd28fb6ee03ee6a328d8bf106c11c2a83d72ed67807f50308a8",
      "bytes": 30031,
      "reason": "Reviewed historical command/search output"
    },
    {
      "path": "w20-semio-tabular-verify/attempt1-mutate-semio-text.txt",
      "sha256": "bc34c59df6165728d0373d15ece76609047b4cf230836befd616b1016f68b01b",
      "bytes": 29037,
      "reason": "Reviewed historical command/search output"
    },
    {
      "path": "w20-semio-tabular-verify/warm-model-build.txt",
      "sha256": "dbdcafbbf229ebc01f390f9ab22306e79191c61ea9c009ce54a4c4aca5c2304c",
      "bytes": 200616,
      "reason": "Reviewed historical command/search output"
    },
    {
      "path": "w20-semio-tabular-verify/oracle-mutate-semio-value.txt",
      "sha256": "36c01f016e5507749b760f7db164c45d1a57940c2bbfafba3bbf298186174844",
      "bytes": 84,
      "reason": "Reviewed historical command/search output"
    },
    {
      "path": "w20-semio-tabular-verify/unit-triples.txt",
      "sha256": "0428e9db765b7cea981b6e5ef86c12413a72970c15b04be92ace69af1b767126",
      "bytes": 200,
      "reason": "Reviewed historical command/search output"
    },
    {
      "path": "w20-semio-tabular-verify/postrevert-mutate-semio-table.txt",
      "sha256": "1ac216d1bda0deac167892dc3afcb3df3db039a17c541ba223f2e2dc49040f92",
      "bytes": 84,
      "reason": "Reviewed historical command/search output"
    },
    {
      "path": "w20-semio-tabular-verify/negctl-mutate-semio-flow.txt",
      "sha256": "d703169ad6fa01a1107e77f2eb94503aae220fbe5511178909082c28e15592fe",
      "bytes": 84,
      "reason": "Reviewed historical command/search output"
    },
    {
      "path": "w20-semio-tabular-verify/attempt3-mutate-semio-text.txt",
      "sha256": "568a1a895109ec278da9a8f429aefb5d949f155013fc7970bf7c67925c23fa57",
      "bytes": 2897,
      "reason": "Reviewed historical command/search output"
    },
    {
      "path": "w20-semio-tabular-verify/oracle-mutate-semio-model.txt",
      "sha256": "307287b0a507e0d7497e0870936afdc6d21af3371f5881fcd02c9889dc921b8d",
      "bytes": 84,
      "reason": "Reviewed historical command/search output"
    },
    {
      "path": "w20-semio-tabular-verify/postrevert-mutate-semio-flow.txt",
      "sha256": "33488a2c1d58b9047af079dc3307709444eccde128df8b6462eafa9f9c4fad0d",
      "bytes": 84,
      "reason": "Reviewed historical command/search output"
    },
    {
      "path": "w20-semio-tabular-verify/negctl-model.txt",
      "sha256": "15125219bfcf4820e7fb46ca08f6f86fd3f4fcbe2892f7d9c3f10d7b39f896fd",
      "bytes": 84,
      "reason": "Reviewed historical command/search output"
    },
    {
      "path": "🧪️w13-mutate-semio-text-cross-language.txt",
      "sha256": "67fbe2d7fc4ac03431a876c8485162627c8e4c06d7852854a767c23a19fd65e2",
      "bytes": 7338,
      "reason": "Reviewed historical command/search output"
    },
    {
      "path": "🧪️w16-oracle-pdf-cargo-tests.txt",
      "sha256": "7b688ab41ec3f166e7332e664f5e16b22ee684f4182f0bfe6eef0d733c7a1a38",
      "bytes": 8103,
      "reason": "Reviewed historical command/search output"
    },
    {
      "path": "🧪️w13-txt-subject-1.txt",
      "sha256": "ca698bb5358ec3f21d50000ce3dd2c80e7131879487d2e3d6c9f1d757df6edac",
      "bytes": 84,
      "reason": "Reviewed historical command/search output"
    },
    {
      "path": "w17-parity-out/mutate-jpg-jfif-1-01-baseline.txt",
      "sha256": "e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855",
      "bytes": 0,
      "reason": "Reviewed historical command/search output"
    },
    {
      "path": "w17-parity-out/mutate-docx-ecma-376-strict.txt",
      "sha256": "e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855",
      "bytes": 0,
      "reason": "Reviewed historical command/search output"
    },
    {
      "path": "w17-parity-out/mutate-dwg-ac1018.txt",
      "sha256": "6718d4cb859545f1f14665c053e890109e0e31ce07fc303a7b9dd1db9eb336ca",
      "bytes": 82,
      "reason": "Reviewed historical command/search output"
    },
    {
      "path": "w17-parity-out/mutate-dwg-ac1024.txt",
      "sha256": "937531ecd3c4e73019b288cb1f07f0d15f8e3ce9c61644fd16bdac58dde164c8",
      "bytes": 25940,
      "reason": "Reviewed historical command/search output"
    },
    {
      "path": "w17-parity-out/mutate-json-rfc8259-i-json.txt",
      "sha256": "06095c2c47d64a682135bcf597a76445435ecf9e6f41275cf29774e6ad565ca7",
      "bytes": 3642,
      "reason": "Reviewed historical command/search output"
    },
    {
      "path": "w17-parity-out/mutate-tiff-6-0-baseline.txt",
      "sha256": "5a41f392b5cffac41575347e9b876b412dd54e1c9eab02c427a403e20b0a36ed",
      "bytes": 84,
      "reason": "Reviewed historical command/search output"
    },
    {
      "path": "🧪️w16-pdf-conformance-parity-after.txt",
      "sha256": "fd797a77b2236dc569a2959e6ef04e26f327907dcd90724ea605d0e06b43fc0a",
      "bytes": 1248,
      "reason": "Reviewed historical command/search output"
    },
    {
      "path": "🧪️w13-contract.txt",
      "sha256": "0d2799b42c24ad7a7ee3d3215fc9c595e89829b16586dd5000c4ffc3269f506f",
      "bytes": 3781,
      "reason": "Reviewed historical command/search output"
    },
    {
      "path": "w22-group-a/🧪️contract-after-program.txt",
      "sha256": "6a188d67cfe3241eda82c6a71d7d507478f5137c6d922ed86f6df7e5a8ee773d",
      "bytes": 1830,
      "reason": "Reviewed historical command/search output"
    },
    {
      "path": "w22-group-a/🧪️oracle-repowide-2.txt",
      "sha256": "e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855",
      "bytes": 0,
      "reason": "Reviewed historical command/search output"
    },
    {
      "path": "w22-group-a/survey.err",
      "sha256": "e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855",
      "bytes": 0,
      "reason": "Tool result stream or stderr"
    },
    {
      "path": "w22-group-a/🧪️oracle-program.txt",
      "sha256": "49916b6d754017d18239f5a3f05542daa4b84e32ea65bd75bd56e9ab1143bd34",
      "bytes": 86,
      "reason": "Reviewed historical command/search output"
    },
    {
      "path": "w22-group-a/📊️census.txt",
      "sha256": "e5d2a08aaf380a89e92dfd8935b3a27239218ea73e19e5c99a66da2ba4bf26db",
      "bytes": 1991,
      "reason": "Reviewed historical command/search output"
    },
    {
      "path": "w22-group-a/🧪️parity-probe.txt",
      "sha256": "71707191c0d5dd544090c63bb7b236441eb71068e7aaa6435cc81507432b95b3",
      "bytes": 2879,
      "reason": "Reviewed historical command/search output"
    },
    {
      "path": "w22-group-a/🧪️oracle-repowide.txt",
      "sha256": "60d37f8ebb52004ac3c920c1e7622e5f52af0a3d59148e27853c9422c271449f",
      "bytes": 2831,
      "reason": "Reviewed historical command/search output"
    },
    {
      "path": "w22-group-a/survey.json",
      "sha256": "dca81fca8b6f8b06ebc883cc5a8ae47506af99d6b602a5fe7e4ba7ad593b633a",
      "bytes": 558987,
      "reason": "Historical generated audit inventory"
    },
    {
      "path": "w22-group-a/🧪️contract-final.txt",
      "sha256": "ef324681ff9c40df806c3d1572dfec189c61c433f266f391d4cc360f0f7e26a5",
      "bytes": 459,
      "reason": "Reviewed historical command/search output"
    },
    {
      "path": "w22-group-a/survey2.json",
      "sha256": "209b2ccf003afc19a711559aba17a303e025b21ca009693b00bc0cd238eea8fc",
      "bytes": 129618,
      "reason": "Historical generated audit inventory"
    },
    {
      "path": "w22-group-a/🧪️oracle-converted-owners.txt",
      "sha256": "0180e98136713509cce8a041b24e3fdd383ce21ec9f7059624a922158e37c624",
      "bytes": 1184,
      "reason": "Reviewed historical command/search output"
    },
    {
      "path": "w23-ifc-differential/parity-evidence.txt",
      "sha256": "397a796cc2bbed1b86a4dc968428999a23e407a7816bb457650406f8bfc89401",
      "bytes": 8105,
      "reason": "Reviewed historical command/search output"
    },
    {
      "path": "w23-ifc-differential/parity-exact-diff.txt",
      "sha256": "883f269acbea4c155733f03b2bb10f8954d5ca2d7d56d614d977a7780c965a6f",
      "bytes": 2599,
      "reason": "Reviewed historical command/search output"
    },
    {
      "path": "w23-ifc-differential/oracle-projections.txt",
      "sha256": "d3b480d9baf50ecccd8ea234c123a418ea319fd98f7c99010677743cc723c0e3",
      "bytes": 5443,
      "reason": "Reviewed historical command/search output"
    },
    {
      "path": "🧪️w13-mutate-txt-utf-8-subject-1.txt",
      "sha256": "e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855",
      "bytes": 0,
      "reason": "Reviewed historical command/search output"
    },
    {
      "path": "w12-norm/survey.json",
      "sha256": "3ce564626b14b03a48d90d85bb46fe9d2a1772897c70c7bde5f09dc689a5601f",
      "bytes": 127668,
      "reason": "Historical generated audit inventory"
    },
    {
      "path": "w18-mutation-fixture-completeness/📄️gaps.json",
      "sha256": "7e20c72a10e327f0bd4beb5caae8545e006ffa4e6212f4f758018a30630c1b26",
      "bytes": 169954,
      "reason": "Historical generated coverage inventory"
    },
    {
      "path": "w18-mutation-fixture-completeness/📄️cargo-stdio-baseline.txt",
      "sha256": "965ac6cc48d5e981571e0c2ca91344bef0b6a892989605e43caaf8fe92082ba5",
      "bytes": 340456,
      "reason": "Reviewed historical command/search output"
    },
    {
      "path": "w18-mutation-fixture-completeness/📄️coverage-gaps.json",
      "sha256": "4f53cda18c2baa0c0354bb5f9a3ecbe5ed12ab4d8e11ba873c2f11161202b945",
      "bytes": 2,
      "reason": "Historical generated coverage inventory"
    },
    {
      "path": "w18-mutation-fixture-completeness/📄️ts-suite.txt",
      "sha256": "b31f16ff3cb2f1748098c9de4a77f012087464efbe3cc8c3dcc978adf51aa83a",
      "bytes": 145516,
      "reason": "Reviewed historical command/search output"
    },
    {
      "path": "w18-mutation-fixture-completeness/📄️contract-baseline.txt",
      "sha256": "79face2a85fc2d0d8764f008c411ac3e83dbf814f8b28900f46ae4aacfe1a70e",
      "bytes": 111347,
      "reason": "Reviewed historical command/search output"
    },
    {
      "path": "w18-mutation-fixture-completeness/📄️stdio-check.txt",
      "sha256": "06be0e00b2a42fd528072574f0cc39ed886d4a29ee476892c302375b3e313829",
      "bytes": 369340,
      "reason": "Reviewed historical command/search output"
    },
    {
      "path": "w18-mutation-fixture-completeness/📄️families.json",
      "sha256": "2681381470c71b8897939d6639aa60edf5d2de7e27283085da70341ee07c24f8",
      "bytes": 1101,
      "reason": "Historical generated coverage inventory"
    },
    {
      "path": "w18-mutation-fixture-completeness/📄️detail.json",
      "sha256": "bca5902dfd0d00683e4c794215672d79a9f68d06c51229b8fc4d7526a9cb5c75",
      "bytes": 68212,
      "reason": "Historical generated coverage inventory"
    },
    {
      "path": "w18-mutation-fixture-completeness/📄️converter-build.txt",
      "sha256": "5d032071b4ca4d8144b91798cc7b8d79116da917afe8602353bdcd15a5004e0e",
      "bytes": 307072,
      "reason": "Reviewed historical command/search output"
    },
    {
      "path": "🧪️w13-mutate-json-rfc8259-subject-1.txt",
      "sha256": "ae288ee2488376837ab665b3d4701c01b3a526fa9e467151850c0c01fe5a5bf1",
      "bytes": 84,
      "reason": "Reviewed historical command/search output"
    },
    {
      "path": "w16-cross-language/🧪️w16-verification.txt",
      "sha256": "6bb8096a939c1fe34fb6a8e18441fad00a264722fe1e1b9124efea389b9780ec",
      "bytes": 4022,
      "reason": "Reviewed historical command/search output"
    },
    {
      "path": "🧪️w13-html5-subject-2.txt",
      "sha256": "bbffc181e2154a3b1ddcd21238bcfc5cbf22ff511f18d21ba538069794aa4c40",
      "bytes": 84,
      "reason": "Reviewed historical command/search output"
    },
    {
      "path": "w13-unblock-evidence/📊️par-puzzle.txt",
      "sha256": "a8c895ddc0d26fee049d57d6f6d5868aeb687714f1440680a60df4f4e03a18a3",
      "bytes": 84,
      "reason": "Reviewed historical command/search output"
    },
    {
      "path": "w13-unblock-evidence/📊️chk-forms.txt",
      "sha256": "0f7f4bae3af842b52bfc638ac9ffa4da2ed6932bfafd36c7cfe8d3b8b1f0ae95",
      "bytes": 364830,
      "reason": "Reviewed historical command/search output"
    },
    {
      "path": "w13-unblock-evidence/📊️final-puzzle.txt",
      "sha256": "a5f164189cef8e3db5fb2c9b0bff0c2cdea5be0cc6c0b05b1cb7e970dea0b01b",
      "bytes": 149518,
      "reason": "Reviewed historical command/search output"
    },
    {
      "path": "w13-unblock-evidence/📊️subj-gis.txt",
      "sha256": "81c9d9701ee3ed8996a064884727bf1fbbfb83df3c4f509b55ab1c027e5a06b1",
      "bytes": 84,
      "reason": "Reviewed historical command/search output"
    },
    {
      "path": "w13-unblock-evidence/📊️final-forms.txt",
      "sha256": "8f7ba9be3b55aaf8eeeaefdc06a7e39ad8fb9e319262c84e962ec4e1b9cd4c51",
      "bytes": 365462,
      "reason": "Reviewed historical command/search output"
    },
    {
      "path": "w13-unblock-evidence/📊️par-gis.txt",
      "sha256": "81c9d9701ee3ed8996a064884727bf1fbbfb83df3c4f509b55ab1c027e5a06b1",
      "bytes": 84,
      "reason": "Reviewed historical command/search output"
    },
    {
      "path": "w13-unblock-evidence/📊️chk-block.txt",
      "sha256": "306eabf9cf6f48960cf955d12149eca5a35a8c1fe5b4fa260f5d81ab113e75b4",
      "bytes": 609249,
      "reason": "Reviewed historical command/search output"
    },
    {
      "path": "w13-unblock-evidence/📊️final-gis.txt",
      "sha256": "db4014f9640e09e8650b8abfdd9be79c723b6df59ee738f2cdc1ca257a49b904",
      "bytes": 113228,
      "reason": "Reviewed historical command/search output"
    },
    {
      "path": "w13-unblock-evidence/📊️subj-puz5.txt",
      "sha256": "233b4542b37398323ebd945ec97e851e6e0696c66cdb4b4a344a18d6d11a468e",
      "bytes": 399715,
      "reason": "Reviewed historical command/search output"
    },
    {
      "path": "w13-unblock-evidence/📊️chk-architect.txt",
      "sha256": "e44ed04a14c1193659524e783907d33be16327b01043402f2c11d782cfb8e1b4",
      "bytes": 962244,
      "reason": "Reviewed historical command/search output"
    },
    {
      "path": "w13-unblock-evidence/📊️par-pdf17.txt",
      "sha256": "436d92c0508733617a206d63634a505ec98ae9e50d1c061626fb3a9226bb7413",
      "bytes": 567,
      "reason": "Reviewed historical command/search output"
    },
    {
      "path": "w13-unblock-evidence/📊️final-dag.txt",
      "sha256": "bc2fa5bf418db10d11a083591fb79c5b2621fb940ba4c1385d7d676997ad5b56",
      "bytes": 276635,
      "reason": "Reviewed historical command/search output"
    },
    {
      "path": "w13-unblock-evidence/📊️subj-puz3.txt",
      "sha256": "8403e396d2071689da35e45770c6343308afdadde5abd8e1ef74a94e68aa38db",
      "bytes": 216712,
      "reason": "Reviewed historical command/search output"
    },
    {
      "path": "w13-unblock-evidence/📊️chk-norm.txt",
      "sha256": "0ce027e238bd21daf6b5af3e227e3613959b23deb7779322edbc3003e17e3136",
      "bytes": 2023041,
      "reason": "Reviewed historical command/search output"
    },
    {
      "path": "w13-unblock-evidence/📊️final-cad.txt",
      "sha256": "ff66f0f4fa56781ab18e7a64a4f1aa2794af8a05af48b17604efb34274b0194b",
      "bytes": 75998,
      "reason": "Reviewed historical command/search output"
    },
    {
      "path": "w13-unblock-evidence/📊️subj-gis2.txt",
      "sha256": "d44876b1d518bd4cd44d025016be12f0256479e41a1e4cf7c4988ee423e14f2f",
      "bytes": 236875,
      "reason": "Reviewed historical command/search output"
    },
    {
      "path": "w13-unblock-evidence/📊️par-cad.txt",
      "sha256": "c3c728f343b0d6b78a6fc4ef1c85a3e0599a12c0b33bf43035c9de5e04f611ad",
      "bytes": 84,
      "reason": "Reviewed historical command/search output"
    },
    {
      "path": "w13-unblock-evidence/📊️chk-dag.txt",
      "sha256": "a9f207aa5050c64bb90d35d62a23b75c60e6f407db0fbb6edf9ba44661c4ff79",
      "bytes": 276094,
      "reason": "Reviewed historical command/search output"
    },
    {
      "path": "w13-unblock-evidence/📊️subj-puzzle.txt",
      "sha256": "a8c895ddc0d26fee049d57d6f6d5868aeb687714f1440680a60df4f4e03a18a3",
      "bytes": 84,
      "reason": "Reviewed historical command/search output"
    },
    {
      "path": "w13-unblock-evidence/📊️subj-cad.txt",
      "sha256": "c3c728f343b0d6b78a6fc4ef1c85a3e0599a12c0b33bf43035c9de5e04f611ad",
      "bytes": 84,
      "reason": "Reviewed historical command/search output"
    },
    {
      "path": "w16-audit/population.txt",
      "sha256": "c6919c80eab9cb1cb8aad738f5c944ae5f04547d050ae3edc453366d6e4ba2ac",
      "bytes": 2135,
      "reason": "Reviewed historical command/search output"
    },
    {
      "path": "w16-audit/02-oracle-repowide.txt",
      "sha256": "8f9c5ee24b17f6a77e7c9c48c5ffaa1091972eb8ec8508008f9bc15601f44e18",
      "bytes": 2855,
      "reason": "Reviewed historical command/search output"
    },
    {
      "path": "w16-audit/plugin-matrix.txt",
      "sha256": "d4b30fb609551f2f8719039d59a48a66ef6471e3b94796319edfc9ec5e1e47cb",
      "bytes": 1379,
      "reason": "Reviewed historical command/search output"
    },
    {
      "path": "w16-audit/03b-parity-stdio.txt",
      "sha256": "ad6fb348716dcbfe18105d5d96738895c5a32d0cb395e126fe8930fa8e958cdf",
      "bytes": 2875,
      "reason": "Reviewed historical command/search output"
    },
    {
      "path": "w16-audit/06-stdio-lib-test.txt",
      "sha256": "df5a95911e9fc71bf3273ad805febdf50a48719e3ebeef2c44da19818546f328",
      "bytes": 475205,
      "reason": "Reviewed historical command/search output"
    },
    {
      "path": "w16-audit/unmanaged.txt",
      "sha256": "529f7938054bb6acd8cfd4f7771881b97b93704c9c19dfbc9c11e6fbde479079",
      "bytes": 7350,
      "reason": "Reviewed historical command/search output"
    },
    {
      "path": "w16-audit/percase/mutate-dxf-r12.txt",
      "sha256": "78049ed5a2a29725c8e0ab0168fb862a919f2c1a15e36bda2c855f0effd38127",
      "bytes": 93,
      "reason": "Reviewed historical command/search output"
    },
    {
      "path": "w16-audit/percase/mutate-deflate-rfc1950.txt",
      "sha256": "e5d7eba0e82175698e2ab285c79f01ad0027313a5255ba322cbd4a614e5b0029",
      "bytes": 93,
      "reason": "Reviewed historical command/search output"
    },
    {
      "path": "w16-audit/percase/mutate-docx-ecma-376-strict.txt",
      "sha256": "7b466f37f7b4fbc3839d65db57696300a90bef73d225a69097dd16a09b1ca4f6",
      "bytes": 93,
      "reason": "Reviewed historical command/search output"
    },
    {
      "path": "w16-audit/percase/create-and-round-trip-png.txt",
      "sha256": "9a5d84ef4ae1a3d8568ca055732de930f35ac48581785cac82ed3b1f825c26da",
      "bytes": 89,
      "reason": "Reviewed historical command/search output"
    },
    {
      "path": "w16-audit/percase/mutate-docx-ecma-376-transitional.txt",
      "sha256": "5b612ce5036655e72b2c1d0852ce3da40beb3aeb0fb43fef9696d21c21674f44",
      "bytes": 93,
      "reason": "Reviewed historical command/search output"
    },
    {
      "path": "w16-audit/percase/mutate-csv-rfc4180.txt",
      "sha256": "5b612ce5036655e72b2c1d0852ce3da40beb3aeb0fb43fef9696d21c21674f44",
      "bytes": 93,
      "reason": "Reviewed historical command/search output"
    },
    {
      "path": "w16-audit/percase/mutate-dwg-ac1018.txt",
      "sha256": "b6b6fd5d0c7b514a833504e51dec6cbd2be34cc5d9599cab240253bed8bfe8c5",
      "bytes": 2893,
      "reason": "Reviewed historical command/search output"
    },
    {
      "path": "w16-audit/percase/mutate-dwg-ac1024.txt",
      "sha256": "6b43b9035b517d9320679a012fb07e8b681427c00d0e3eb655696d85cb9a39d9",
      "bytes": 2893,
      "reason": "Reviewed historical command/search output"
    },
    {
      "path": "w16-audit/percase/mutate-bcf-2-1.txt",
      "sha256": "0100900f3c714724abdd7bfca06d169f8db0a4166f7d2cf3d4056b767d7df146",
      "bytes": 93,
      "reason": "Reviewed historical command/search output"
    },
    {
      "path": "w16-audit/percase/edit-existing-pdf.txt",
      "sha256": "345fc086b4ff7cb06601254f790ea727a8637120f630f055476090facbe12cfb",
      "bytes": 89,
      "reason": "Reviewed historical command/search output"
    },
    {
      "path": "w16-audit/percase/mutate-avi-1-0.txt",
      "sha256": "22ae5aa0a95328b2cfae2cc7cf089650d52ca6d3b3459cc20d0a4784f8aa522b",
      "bytes": 93,
      "reason": "Reviewed historical command/search output"
    },
    {
      "path": "w16-audit/percase/create-and-round-trip-stl.txt",
      "sha256": "345fc086b4ff7cb06601254f790ea727a8637120f630f055476090facbe12cfb",
      "bytes": 89,
      "reason": "Reviewed historical command/search output"
    },
    {
      "path": "w16-audit/percase/mutate-epw-energyplus.txt",
      "sha256": "e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855",
      "bytes": 0,
      "reason": "Reviewed historical command/search output"
    },
    {
      "path": "w16-audit/percase/create-and-edit-archive.txt",
      "sha256": "9a5d84ef4ae1a3d8568ca055732de930f35ac48581785cac82ed3b1f825c26da",
      "bytes": 89,
      "reason": "Reviewed historical command/search output"
    },
    {
      "path": "w16-audit/percase/create-and-round-trip-bmp.txt",
      "sha256": "345fc086b4ff7cb06601254f790ea727a8637120f630f055476090facbe12cfb",
      "bytes": 89,
      "reason": "Reviewed historical command/search output"
    },
    {
      "path": "w16-audit/percase/mutate-bmp-v3.txt",
      "sha256": "dc044dcc83dee8ef69ce9f2bc078454c31fa24595464e084a23e21414b869421",
      "bytes": 248,
      "reason": "Reviewed historical command/search output"
    },
    {
      "path": "w16-audit/percase/create-and-round-trip-gif.txt",
      "sha256": "345fc086b4ff7cb06601254f790ea727a8637120f630f055476090facbe12cfb",
      "bytes": 89,
      "reason": "Reviewed historical command/search output"
    },
    {
      "path": "w16-audit/percase/mutate-docx-ecma-376.txt",
      "sha256": "22ae5aa0a95328b2cfae2cc7cf089650d52ca6d3b3459cc20d0a4784f8aa522b",
      "bytes": 93,
      "reason": "Reviewed historical command/search output"
    },
    {
      "path": "w16-audit/percase/create-and-read-jpeg.txt",
      "sha256": "345fc086b4ff7cb06601254f790ea727a8637120f630f055476090facbe12cfb",
      "bytes": 89,
      "reason": "Reviewed historical command/search output"
    },
    {
      "path": "w16-audit/percase/extract-text-pdf-1-4.txt",
      "sha256": "6e3821ab19ef0b3ec7a732b488d6f87450575182c3b791616872cfc8a5ce5e63",
      "bytes": 339,
      "reason": "Reviewed historical command/search output"
    },
    {
      "path": "w16-audit/percase/mutate-gif-87a.txt",
      "sha256": "5c6cf53e4f7a175660090bf10282a621340203f8446ddfcb8019a7532939ae03",
      "bytes": 257,
      "reason": "Reviewed historical command/search output"
    },
    {
      "path": "w16-audit/percase/create-and-round-trip-obj.txt",
      "sha256": "345fc086b4ff7cb06601254f790ea727a8637120f630f055476090facbe12cfb",
      "bytes": 89,
      "reason": "Reviewed historical command/search output"
    },
    {
      "path": "w16-audit/percase/mutate-binary-raw.txt",
      "sha256": "5c2f6221557c4203e7f95153118b22f473661e3a8970ae1270fa8ed417976916",
      "bytes": 91,
      "reason": "Reviewed historical command/search output"
    },
    {
      "path": "w16-audit/percase/create-and-retune-wave.txt",
      "sha256": "9a5d84ef4ae1a3d8568ca055732de930f35ac48581785cac82ed3b1f825c26da",
      "bytes": 89,
      "reason": "Reviewed historical command/search output"
    },
    {
      "path": "w16-audit/percase/create-and-round-trip-tiff.txt",
      "sha256": "345fc086b4ff7cb06601254f790ea727a8637120f630f055476090facbe12cfb",
      "bytes": 89,
      "reason": "Reviewed historical command/search output"
    },
    {
      "path": "w16-audit/percase/create-minimal-pdf.txt",
      "sha256": "9a5d84ef4ae1a3d8568ca055732de930f35ac48581785cac82ed3b1f825c26da",
      "bytes": 89,
      "reason": "Reviewed historical command/search output"
    },
    {
      "path": "w16-audit/03-parity-repowide.txt",
      "sha256": "b3cee4672e193c35cc5d6ffffbcaa441f03f193774735c1ee7f1b2785ad01ef5",
      "bytes": 2923,
      "reason": "Reviewed historical command/search output"
    },
    {
      "path": "w16-audit/04-dependency.txt",
      "sha256": "91e5a92e6418570d3daa6b202b5018b8d95e70c686ad315a922847a3b1b993d0",
      "bytes": 4101,
      "reason": "Reviewed historical command/search output"
    },
    {
      "path": "w16-audit/01-contract.txt",
      "sha256": "802222476b743b4bc04ed80ad28f54e8e11165c871008abb6956a1845cde4964",
      "bytes": 459,
      "reason": "Reviewed historical command/search output"
    },
    {
      "path": "w16-audit/05-ts-suite.txt",
      "sha256": "d8c7ca0ccc16834a2afb93c935aeb943262f96139cc0575ceadf49c6cfa8b3fb",
      "bytes": 2396,
      "reason": "Reviewed historical command/search output"
    },
    {
      "path": "w16-audit/07-parity-kernel.txt",
      "sha256": "dd2e9a3c50146c6d4d454f345f0be6a260ff1b32748313aeba0090810cc40c2b",
      "bytes": 82,
      "reason": "Reviewed historical command/search output"
    },
    {
      "path": "w16-audit/10-warm-host-build.txt",
      "sha256": "d10128b9f1ffff83bf5ecf6146606c42876905253913877716661937b2c9ba95",
      "bytes": 186811,
      "reason": "Reviewed historical command/search output"
    },
    {
      "path": "w16-audit/07-stdio-oracle-tests.txt",
      "sha256": "60d1a054e6b7d696deb3e47d4b67e619526d2beff8e1c6d870f17ea3742161e4",
      "bytes": 51861,
      "reason": "Reviewed historical command/search output"
    },
    {
      "path": "w16-audit/08-stdio-check.txt",
      "sha256": "7bf0490bee080abb7d174401d4ef7f51c107a7ab813e49b541040914de3164de",
      "bytes": 176871,
      "reason": "Reviewed historical command/search output"
    },
    {
      "path": "w16-audit/06-parity-ui.txt",
      "sha256": "2625732b9aa1246e572e5a51921e593cf2fa6bffa80864ec8c601a19ff67d5d3",
      "bytes": 84,
      "reason": "Reviewed historical command/search output"
    },
    {
      "path": "w16-audit/09-framework-plugin-check.txt",
      "sha256": "3353b8c97e18293f73648a1afde409289e799efb40066bb4238a5868e6c54cd3",
      "bytes": 29211,
      "reason": "Reviewed historical command/search output"
    },
    {
      "path": "🧪️w13-mutate-md-commonmark-subject-1.txt",
      "sha256": "e4941399f8b061032313887e5a59956966e6d1c2f6906104fa390b78a8a7ebd8",
      "bytes": 83,
      "reason": "Reviewed historical command/search output"
    },
    {
      "path": "🧪️w13-md-subject-2.txt",
      "sha256": "7631d23fe114ede9d7948f67a42d2b876075ed79275f43ce0924fbc6baf22797",
      "bytes": 84,
      "reason": "Reviewed historical command/search output"
    },
    {
      "path": "w14-audit/population.txt",
      "sha256": "5aa1367c36ce0746ef631b5901de6a22da1759699aae9676ad53912e9492a47f",
      "bytes": 259,
      "reason": "Reviewed historical command/search output"
    },
    {
      "path": "w14-audit/02-oracle-repowide.txt",
      "sha256": "049aae73c3dc68cbce197c031c88981b74c7e1e412107068177c988766f9b952",
      "bytes": 15087,
      "reason": "Reviewed historical command/search output"
    },
    {
      "path": "w14-audit/fixtures2.json",
      "sha256": "904d9ce36a0d9231b22a5d1b63adca68a0a0286f673a61b32ec950e0bd72f003",
      "bytes": 3972461,
      "reason": "Historical generated audit inventory"
    },
    {
      "path": "w14-audit/fixtures.err",
      "sha256": "e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855",
      "bytes": 0,
      "reason": "Tool result stream or stderr"
    },
    {
      "path": "w14-audit/survey.err",
      "sha256": "e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855",
      "bytes": 0,
      "reason": "Tool result stream or stderr"
    },
    {
      "path": "w14-audit/06-cargo-oracles.txt",
      "sha256": "dcaa954391b6ff16039e18f3d8156e48cf5fda8bd58abc1fa31dea40dfa9a5c2",
      "bytes": 51506,
      "reason": "Reviewed historical command/search output"
    },
    {
      "path": "w14-audit/03d-parity-probe-en1990.txt",
      "sha256": "28a759e487293638a99db2e48e7fef2b5ae5f3d25077a9a243be4aeaa7e1dcd7",
      "bytes": 111855,
      "reason": "Reviewed historical command/search output"
    },
    {
      "path": "w14-audit/03-parity-repowide-partial.txt",
      "sha256": "349fa6491d9a99d4421f70f32c10f60a565611fe8d822cbf247dc760cde7cb2a",
      "bytes": 4,
      "reason": "Reviewed historical command/search output"
    },
    {
      "path": "w14-audit/03c-parity-probe-zip.txt",
      "sha256": "51dda3b4b58411f4ba5391328572fa774fe861b6dd0a95cf02e1ba9130c505f8",
      "bytes": 92223,
      "reason": "Reviewed historical command/search output"
    },
    {
      "path": "w14-audit/04-dependency.txt",
      "sha256": "91e5a92e6418570d3daa6b202b5018b8d95e70c686ad315a922847a3b1b993d0",
      "bytes": 4101,
      "reason": "Reviewed historical command/search output"
    },
    {
      "path": "w14-audit/01-contract.txt",
      "sha256": "ef324681ff9c40df806c3d1572dfec189c61c433f266f391d4cc360f0f7e26a5",
      "bytes": 459,
      "reason": "Reviewed historical command/search output"
    },
    {
      "path": "w14-audit/no-oracle.txt",
      "sha256": "a407ecc536b95a4c53ad32fb9fbe2a716ef55476f98104b47a06e879de49fe75",
      "bytes": 39783,
      "reason": "Reviewed historical command/search output"
    },
    {
      "path": "w14-audit/05-ts-suite.txt",
      "sha256": "3314f290f3ec4e251618e8d07443d4d92b9cb4a0671b7e4ec317228a6e6ed428",
      "bytes": 2227,
      "reason": "Reviewed historical command/search output"
    },
    {
      "path": "w14-audit/survey.json",
      "sha256": "00770b9d3b46a83b9b4ec98dc4c5ec2e8b01577de4cfedb67acbc2609cca63ea",
      "bytes": 431179,
      "reason": "Historical generated audit inventory"
    },
    {
      "path": "w14-audit/fixtures.json",
      "sha256": "ae5f715122d215518fc3d22775e42e55ea3c94aa676c24ecd7b3a693215b949d",
      "bytes": 2653174,
      "reason": "Historical generated audit inventory"
    },
    {
      "path": "w18-pdf14-and-part21-header/retry.txt",
      "sha256": "8b3d5dc008eb5a649f2d2f31791fcc2b109cb9c5fe1ea1410022b72a5ba49a32",
      "bytes": 2906,
      "reason": "Reviewed historical command/search output"
    },
    {
      "path": "w18-pdf14-and-part21-header/contract.txt",
      "sha256": "802222476b743b4bc04ed80ad28f54e8e11165c871008abb6956a1845cde4964",
      "bytes": 459,
      "reason": "Reviewed historical command/search output"
    },
    {
      "path": "w18-pdf14-and-part21-header/cases.txt",
      "sha256": "5434388586280a471fc6169a5f0fb8d4da36686fe3aee780d8425795b9cfef81",
      "bytes": 118240,
      "reason": "Reviewed historical command/search output"
    },
    {
      "path": "w18-pdf14-and-part21-header/retry2.txt",
      "sha256": "3635208cba6ba050a0353867600b75be740e849a3900b049cc79680ce9499f99",
      "bytes": 521,
      "reason": "Reviewed historical command/search output"
    },
    {
      "path": "w17-writer-fidelity/prebuild5.txt",
      "sha256": "9b186f8ac71846921221fccfe5071b06adc2bb71f756d673f3b72ce3c3a0666b",
      "bytes": 1765,
      "reason": "Reviewed historical command/search output"
    },
    {
      "path": "w17-writer-fidelity/parity-run3.txt",
      "sha256": "db86c2836735acd704c8f150c8a4ff5988f8e50b1bc514a898eb92b71505af3e",
      "bytes": 133,
      "reason": "Reviewed historical command/search output"
    },
    {
      "path": "w7-semio-model-presentation/📊️verification.txt",
      "sha256": "0d859a49c3cee5f265f73f0ddd4a2ab500003624aef6b1958813ce8e1c17ff91",
      "bytes": 5693,
      "reason": "Reviewed historical command/search output"
    },
    {
      "path": "🧪️w13-txt-parity-1.txt",
      "sha256": "ca698bb5358ec3f21d50000ce3dd2c80e7131879487d2e3d6c9f1d757df6edac",
      "bytes": 84,
      "reason": "Reviewed historical command/search output"
    },
    {
      "path": "w17-crosscutting/12.json",
      "sha256": "e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855",
      "bytes": 0,
      "reason": "Historical generated audit inventory"
    },
    {
      "path": "🧪️w13-mutate-svg-1-1-parity-2.txt",
      "sha256": "e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855",
      "bytes": 0,
      "reason": "Reviewed historical command/search output"
    },
    {
      "path": "🧪️w13-svg-parity-2.txt",
      "sha256": "4876ff952ca17323a72166e0b179a6f79f3eedfc62698e3af344d626ca51d307",
      "bytes": 86,
      "reason": "Reviewed historical command/search output"
    },
    {
      "path": "🧪️w13-md-parity-2.txt",
      "sha256": "913906c4efaf59a7461ee47fb37a68f8caa7548601eb01ece84cb766ccd48f4d",
      "bytes": 246,
      "reason": "Reviewed historical command/search output"
    },
    {
      "path": "w15-audit/oracle-sweep/➗️mathematical.txt",
      "sha256": "8323b5587e746ea2a4d929bc350b8b25fa730e01bded1f70655e01ebd914d285",
      "bytes": 353,
      "reason": "Reviewed historical command/search output"
    },
    {
      "path": "w15-audit/oracle-sweep/📜️imperative.txt",
      "sha256": "3f224b93c0156cf62a5061b472c9fcd68afb7ef8f727cdd714848a1a880b12d1",
      "bytes": 366,
      "reason": "Reviewed historical command/search output"
    },
    {
      "path": "w15-audit/oracle-sweep/🧰️framework-🛍️products-💻️os.txt",
      "sha256": "bcaa9faef134b9e9860150b2548db7595e20b607d07fd2db7ea24d60e016c047",
      "bytes": 379,
      "reason": "Reviewed historical command/search output"
    },
    {
      "path": "w15-audit/oracle-sweep/✒️writer.txt",
      "sha256": "b862a520be0dba93ae8821f9ecae77200a6fec9552dec5d8c6a7bcee87826148",
      "bytes": 89,
      "reason": "Reviewed historical command/search output"
    },
    {
      "path": "w15-audit/oracle-sweep/🪵️sourcing.txt",
      "sha256": "79fe9696989f19bedf82c765feb3046accdeadd3dfe86fc246614fcbce4606e4",
      "bytes": 91,
      "reason": "Reviewed historical command/search output"
    },
    {
      "path": "w15-audit/oracle-sweep/🧰️framework-🔨️modules-🖱️ui.txt",
      "sha256": "380944cb4746de0f37f7a610be906fc20be9db5c5139bf0ba9e8207a68407419",
      "bytes": 368,
      "reason": "Reviewed historical command/search output"
    },
    {
      "path": "w15-audit/oracle-sweep/🎞️animate.txt",
      "sha256": "e24db42e9a6228d3e0c3cbd2fc9f7377f5943184e386e27b48695d7ef6353b58",
      "bytes": 347,
      "reason": "Reviewed historical command/search output"
    },
    {
      "path": "w15-audit/oracle-sweep/🎬️sequence.txt",
      "sha256": "f6f200d29c91b97ccaf6a09832a2b526ad5a3afadac4d0c0ecf8a33cab54bfe2",
      "bytes": 350,
      "reason": "Reviewed historical command/search output"
    },
    {
      "path": "w15-audit/oracle-sweep/🔋️energy.txt",
      "sha256": "7ec86fa95152bbd5c214364ef28e21361f9ba7296bfa8764fb63f30007eaee00",
      "bytes": 342,
      "reason": "Reviewed historical command/search output"
    },
    {
      "path": "w15-audit/oracle-sweep/🪐️space.txt",
      "sha256": "5c54e8ec6114c413b25cd9be6e711fbf61d97bbf4c920a5176579cf8178ff443",
      "bytes": 560,
      "reason": "Reviewed historical command/search output"
    },
    {
      "path": "w15-audit/oracle-sweep/🌿️vcs.txt",
      "sha256": "12ba2212600ca2a11f751a20a345029050ebf0ab7a08b37e4662035e2b7e5357",
      "bytes": 332,
      "reason": "Reviewed historical command/search output"
    },
    {
      "path": "w15-audit/oracle-sweep/📏️layout.txt",
      "sha256": "b0aceae5a99e1f01cc241936afab88ccda064b896f0c1112204c86124cf31292",
      "bytes": 331,
      "reason": "Reviewed historical command/search output"
    },
    {
      "path": "w15-audit/oracle-sweep/🕸️dag.txt",
      "sha256": "707a9297e6d4fa45df75a3b871628963ff1c7c606c50ca6f3a85f935187d2286",
      "bytes": 341,
      "reason": "Reviewed historical command/search output"
    },
    {
      "path": "w15-audit/oracle-sweep/🏗️fem.txt",
      "sha256": "0aed6434feb04ecab5de76c43f2153904a7c2665efa66ae0d36e78c0c69be1d8",
      "bytes": 93,
      "reason": "Reviewed historical command/search output"
    },
    {
      "path": "w15-audit/oracle-sweep/🏛️architect.txt",
      "sha256": "76de2cc1e0cdd015160fda28ae7faa17f2022dde5a529c6481449a9b09ffd86b",
      "bytes": 93,
      "reason": "Reviewed historical command/search output"
    },
    {
      "path": "w15-audit/oracle-sweep/🔱️trinity.txt",
      "sha256": "07fd449eb39ac68c9a95f0352462a5dcb0122360df34f32752f8428053753ed6",
      "bytes": 91,
      "reason": "Reviewed historical command/search output"
    },
    {
      "path": "w15-audit/oracle-sweep/🧩️puzzle.txt",
      "sha256": "a2564a6396a995c75476ff82a1211387232bf8e344c7ce600422927a3f428d5a",
      "bytes": 93,
      "reason": "Reviewed historical command/search output"
    },
    {
      "path": "w15-audit/oracle-sweep/🎪️demonstrator.txt",
      "sha256": "deb453145f648e9f2b90f37087218982dd07f85508ebf3f69e0fd011c2976bdc",
      "bytes": 349,
      "reason": "Reviewed historical command/search output"
    },
    {
      "path": "w15-audit/oracle-sweep/📋️forms.txt",
      "sha256": "b5c5e6f0a2a8a112de4823e4f12885a311dafcd39681d327a24a2967347610cf",
      "bytes": 91,
      "reason": "Reviewed historical command/search output"
    },
    {
      "path": "w15-audit/oracle-sweep/📖️playbook.txt",
      "sha256": "fb1c8e3b90d8122d4af21e4a015264dc464766a0503a38a4680cebea34b907d2",
      "bytes": 91,
      "reason": "Reviewed historical command/search output"
    },
    {
      "path": "w15-audit/oracle-sweep/🖨️raster.txt",
      "sha256": "02ecf955270df55dff777a8f88f3799d2446393ad9efb929aa7a1aedc570a548",
      "bytes": 91,
      "reason": "Reviewed historical command/search output"
    },
    {
      "path": "w15-audit/oracle-sweep/📸️remodel.txt",
      "sha256": "439f8162de6b7cd7e5c698dccd35a80ff720741edcefe116cebb57af66e4d9cc",
      "bytes": 335,
      "reason": "Reviewed historical command/search output"
    },
    {
      "path": "w15-audit/oracle-sweep/🌀️procedural.txt",
      "sha256": "34cd9e4a78718a861146919a5724a98245791b87f70986c71dde9893c683e50d",
      "bytes": 91,
      "reason": "Reviewed historical command/search output"
    },
    {
      "path": "w15-audit/oracle-sweep/📕️norm.txt",
      "sha256": "a016a6d25670e53f8e439e1b292940b497dabe3685e16e76f4acae58cf4f786d",
      "bytes": 94,
      "reason": "Reviewed historical command/search output"
    },
    {
      "path": "w15-audit/oracle-sweep/🎥️shooting.txt",
      "sha256": "c7d8dc09d7706292bd07f1dd4576d463743a67cdfeb439c31039a023e6b94222",
      "bytes": 352,
      "reason": "Reviewed historical command/search output"
    },
    {
      "path": "w15-audit/oracle-sweep/🏭️process.txt",
      "sha256": "02ccb5d69bf483f3894736c1607fe76a1959491cd6e89f9ba5f27286aa054bdc",
      "bytes": 341,
      "reason": "Reviewed historical command/search output"
    },
    {
      "path": "w15-audit/oracle-sweep/🗒️note.txt",
      "sha256": "bb6548992d2538ba27d513d10538207f753faa25174522ce6a1e6f1b04f96ab6",
      "bytes": 91,
      "reason": "Reviewed historical command/search output"
    },
    {
      "path": "w15-audit/oracle-sweep/📐️cad.txt",
      "sha256": "13a409098a0fd72a25247800c19283fb14c772c61cae97f3dc27899bfd54c2b0",
      "bytes": 91,
      "reason": "Reviewed historical command/search output"
    },
    {
      "path": "w15-audit/oracle-sweep/🧱️block.txt",
      "sha256": "7ff5ca751a4111c9bc34ff459bcbbbdfe4f3c6ceceac449ccfd4d6f86b41e180",
      "bytes": 93,
      "reason": "Reviewed historical command/search output"
    },
    {
      "path": "w15-audit/oracle-sweep/🌍️gis.txt",
      "sha256": "cd60b96784c9f02a3dc3957bbe0be4e26e08faef72350400afcf1d4690138022",
      "bytes": 91,
      "reason": "Reviewed historical command/search output"
    },
    {
      "path": "w15-audit/oracle-sweep/💡️reasoning.txt",
      "sha256": "1c67d5a669a70291b8323ef2abbb704b1de18dad0350e1c77325854cdc3171b2",
      "bytes": 348,
      "reason": "Reviewed historical command/search output"
    },
    {
      "path": "w15-audit/oracle-sweep/💠️lowpoly.txt",
      "sha256": "45320c903f54d68ac916ab45b8e2e6d73001627313303afe8faf177c78f8afb2",
      "bytes": 91,
      "reason": "Reviewed historical command/search output"
    },
    {
      "path": "w15-audit/oracle-sweep/🌊️flow.txt",
      "sha256": "8fc04fa8184dd56f814dc68431196a2961db52dbe5a438f89104223331db08db",
      "bytes": 336,
      "reason": "Reviewed historical command/search output"
    },
    {
      "path": "w15-audit/oracle-sweep/🧰️framework-🛍️products-🦑️repo.txt",
      "sha256": "c98371d0ad2e4bf46359da40efa8353e357a3ba844edc2cf877341951bafcb13",
      "bytes": 333,
      "reason": "Reviewed historical command/search output"
    },
    {
      "path": "w15-audit/oracle-sweep/🖍️draw.txt",
      "sha256": "555c66748493e3298f1d746a0d2e5892307e6be95e98271c0ae69591707a7e33",
      "bytes": 323,
      "reason": "Reviewed historical command/search output"
    },
    {
      "path": "w15-audit/oracle-sweep/🧰️framework-🔨️modules-🎠️kernel.txt",
      "sha256": "e379e10960e7d647ff8564b39989733e4980afa86fc3e1e4cbe0d80bd3998817",
      "bytes": 325,
      "reason": "Reviewed historical command/search output"
    },
    {
      "path": "w15-audit/failures/🏛️architect.jsonl",
      "sha256": "f98a2bbf52e52fe008e0be68baebb75a33773aa59aa83dcef6f65e4ec84701fb",
      "bytes": 14138,
      "reason": "Tool result stream or stderr"
    },
    {
      "path": "w15-audit/failures/📖️playbook.jsonl",
      "sha256": "f195bf43c32ac47559b606b4ff9c57d9bce222b1c91120200584e72877ca1b25",
      "bytes": 3119,
      "reason": "Tool result stream or stderr"
    },
    {
      "path": "w15-audit/failures/📋️forms.jsonl",
      "sha256": "d3d731792c7f062291d449244eecc8606fadbed3409c3bf413e55f1f114dccb7",
      "bytes": 5176,
      "reason": "Tool result stream or stderr"
    },
    {
      "path": "w15-audit/failures/🧱️block.jsonl",
      "sha256": "6542e9171d9caa76734da51a5390c988c013ab6b35890f68fde040d28877e546",
      "bytes": 17418,
      "reason": "Tool result stream or stderr"
    },
    {
      "path": "w15-audit/failures/🗒️note.jsonl",
      "sha256": "b1904d971888a732608fd2b3ec57674d2042a02053b0b5ebd22bac8697b257cf",
      "bytes": 9947,
      "reason": "Tool result stream or stderr"
    },
    {
      "path": "w15-audit/failures/✒️writer.jsonl",
      "sha256": "7d28f9eeacf8dd94d8c86d1e401182b4c1d6135d35026f9b8115a67291ada698",
      "bytes": 9961,
      "reason": "Tool result stream or stderr"
    },
    {
      "path": "w15-audit/failures/🧩️puzzle.jsonl",
      "sha256": "c043659c03ae8bf8169c5e010383ca08a4385d57675c54bce78e33bb4ba160bb",
      "bytes": 18989,
      "reason": "Tool result stream or stderr"
    },
    {
      "path": "w15-audit/owners.txt",
      "sha256": "7d078ccde292ecb74c6a2283a8067af4dfbc11fdb96759bfba5c85f63efbfef2",
      "bytes": 648,
      "reason": "Reviewed historical command/search output"
    },
    {
      "path": "w15-audit/02-oracle-repowide.txt",
      "sha256": "7c49911762aa7da722f988a9f38cdcbb6e9259ddba7f27c87e0e59a6dde9e166",
      "bytes": 2862,
      "reason": "Reviewed historical command/search output"
    },
    {
      "path": "w15-audit/fixtures.err",
      "sha256": "e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855",
      "bytes": 0,
      "reason": "Tool result stream or stderr"
    },
    {
      "path": "w15-audit/report-norm/📊️summary.json",
      "sha256": "f36790efb2cabc74da28faabc99530c66e229a7fd8eb927bcddfb3ce9a17e4d8",
      "bytes": 274,
      "reason": "Historical generated audit inventory"
    },
    {
      "path": "w15-audit/report-norm/📤️results.jsonl",
      "sha256": "ccce38565eba25bfa39ccdb743be3736ede13863b4d17c2e791c55584795776d",
      "bytes": 1837025,
      "reason": "Tool result stream or stderr"
    },
    {
      "path": "w15-audit/02b-oracle-norm.txt",
      "sha256": "3b1f44c0a800cc7eec2cc209da3341de491c27e6aed7cbdbc50e93a9a450043f",
      "bytes": 106,
      "reason": "Reviewed historical command/search output"
    },
    {
      "path": "w15-audit/survey.err",
      "sha256": "e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855",
      "bytes": 0,
      "reason": "Tool result stream or stderr"
    },
    {
      "path": "w15-audit/02-oracle-repowide-attempt2.txt",
      "sha256": "7e3e919bf85895c2d22181bc2f83272b3c44cfffb5091631b87ead4a52b44e1c",
      "bytes": 2838,
      "reason": "Reviewed historical command/search output"
    },
    {
      "path": "w15-audit/06-cargo-oracles.txt",
      "sha256": "1933238e4e3ee06336da3a9e266a9f1e0c6633c426879b2dea7dc084b6bf458b",
      "bytes": 53052,
      "reason": "Reviewed historical command/search output"
    },
    {
      "path": "w15-audit/03c-parity-probe-zip.txt",
      "sha256": "3e055b2421702828092bb0eb7b91ec2b4626c5e78b2faab8760b3321b64b530e",
      "bytes": 6503,
      "reason": "Reviewed historical command/search output"
    },
    {
      "path": "w15-audit/03g-parity-kernel.txt",
      "sha256": "8f0d2a835e8c6bffba7cf491562064079703184d8a175346302cc32448d215e4",
      "bytes": 103,
      "reason": "Reviewed historical command/search output"
    },
    {
      "path": "w15-audit/owners-nonstdio.txt",
      "sha256": "dcf4436894896a2e1f04c011fcbe71e1e17c48566a4dad1cc314437232da733d",
      "bytes": 635,
      "reason": "Reviewed historical command/search output"
    },
    {
      "path": "w15-audit/02c-oracle-jack.txt",
      "sha256": "0f1a08086dd1a0a1ef62e97dba7229f4ca585b9275ad5133ffe13093bae197d1",
      "bytes": 103,
      "reason": "Reviewed historical command/search output"
    },
    {
      "path": "w15-audit/04-dependency.txt",
      "sha256": "d4fae2af1db4071ea5b0969ad50549c499541d6b1b53f9348bc2038c2e7d17dd",
      "bytes": 4252,
      "reason": "Reviewed historical command/search output"
    },
    {
      "path": "w15-audit/01-contract.txt",
      "sha256": "5f6d1cb8bbdb35927f2046845ef411b9034d2518f2c8def328edb69054e402e7",
      "bytes": 475,
      "reason": "Reviewed historical command/search output"
    },
    {
      "path": "w15-audit/05-ts-suite.txt",
      "sha256": "8362982ff8f858d6fc0e183a39237ca269fa97579d88370d58680a8629dabc4e",
      "bytes": 4850,
      "reason": "Reviewed historical command/search output"
    },
    {
      "path": "w15-audit/fixture-rows.json",
      "sha256": "3f66ab403b1ba952b19b5e79af4cafec061c9af1ca23635d52682e02dbb6d763",
      "bytes": 75121,
      "reason": "Historical generated audit inventory"
    },
    {
      "path": "w15-audit/survey.json",
      "sha256": "e3b0de6b9182e0aad9b7e3748ea4c6837883c3098903db38b41ebfbb11fb2676",
      "bytes": 507997,
      "reason": "Historical generated audit inventory"
    },
    {
      "path": "w15-audit/failing-owners.txt",
      "sha256": "69ba1976116b2c68d45319324e081b1917791d275650d52e275a81c24301c9be",
      "bytes": 98,
      "reason": "Reviewed historical command/search output"
    },
    {
      "path": "w15-audit/fixtures.json",
      "sha256": "72b320f3b692b49e33983d15babaf4b06734d9de1647696c93ea7c096af79ea8",
      "bytes": 2663542,
      "reason": "Historical generated audit inventory"
    },
    {
      "path": "w15-audit/03d-parity-probe-zip-retry.txt",
      "sha256": "d01015a88dd57994e6794d91166e1695f74b914e9c9a1eb3e4f8098172ff77ac",
      "bytes": 2892,
      "reason": "Reviewed historical command/search output"
    },
    {
      "path": "w15-audit/03f-parity-ui.txt",
      "sha256": "cf41a4dc597928118a90c2a6f7ac2228d829a823e684596b92ec7f95d18365ce",
      "bytes": 101,
      "reason": "Reviewed historical command/search output"
    },
    {
      "path": "w15-audit/03e-parity-probe-en1990.txt",
      "sha256": "400a2f93637cdce056011338971768e3ea4c597d82c143a2a55b93d4832efc76",
      "bytes": 2907,
      "reason": "Reviewed historical command/search output"
    },
    {
      "path": "🧪️w13-json-parity-1.txt",
      "sha256": "e5a34f31908d75c9a851d1d7ed74ec9dbbac5efcb5e5256027c051e2a499a370",
      "bytes": 86,
      "reason": "Reviewed historical command/search output"
    },
    {
      "path": "🧪️w13-md-parity-1.txt",
      "sha256": "913906c4efaf59a7461ee47fb37a68f8caa7548601eb01ece84cb766ccd48f4d",
      "bytes": 246,
      "reason": "Reviewed historical command/search output"
    },
    {
      "path": "🧪️w13-json-parity-2.txt",
      "sha256": "e5a34f31908d75c9a851d1d7ed74ec9dbbac5efcb5e5256027c051e2a499a370",
      "bytes": 86,
      "reason": "Reviewed historical command/search output"
    },
    {
      "path": "w17-parity-out2/mutate-dwg-ac1018.txt",
      "sha256": "3558c39d510830ccb5cf704c4d7a5be54da5f7a7d23e86d821ed79f4469cdcc4",
      "bytes": 82,
      "reason": "Reviewed historical command/search output"
    },
    {
      "path": "w17-parity-out2/mutate-dwg-ac1024.txt",
      "sha256": "e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855",
      "bytes": 0,
      "reason": "Reviewed historical command/search output"
    },
    {
      "path": "🧑‍💻coordination/🧫️nx-fixture/.nx/workspace-data/source-maps.json",
      "sha256": "d8a4270a19c03408d863aee5c24dbb5041cf1fe288ead2eca16e0a0ca208be3e",
      "bytes": 757,
      "reason": "Nx generated workspace state"
    },
    {
      "path": "🧑‍💻coordination/🧫️nx-fixture/.nx/workspace-data/file-map.json",
      "sha256": "987c72fc88dfeb7b772e775ffd69bf7ba12fed0751be1051c83e089a3c8779b9",
      "bytes": 528,
      "reason": "Nx generated workspace state"
    },
    {
      "path": "🧑‍💻coordination/🧫️nx-fixture/.nx/workspace-data/project-graph.lock",
      "sha256": "e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855",
      "bytes": 0,
      "reason": "Nx generated workspace state"
    },
    {
      "path": "🧑‍💻coordination/🧫️nx-fixture/.nx/workspace-data/project-graph.json",
      "sha256": "fcc77086a7a0b0e58ab523a620e08db2dac798e26f760844fb782c0decfc9e08",
      "bytes": 610,
      "reason": "Nx generated workspace state"
    },
    {
      "path": "w22-fixture-upgrade/measure.json",
      "sha256": "7930e20c6ea39c2a9b7c9b0047b16f4f90b5f84063c3c03083a116e79ccbb3db",
      "bytes": 179891,
      "reason": "Historical generated audit inventory"
    },
    {
      "path": "w22-fixture-upgrade/measure-after.json",
      "sha256": "f8fb96f29d7c6d2b6563e2c44ae505247f439c1e8dd005b999e53b0251bb396c",
      "bytes": 184039,
      "reason": "Historical generated audit inventory"
    },
    {
      "path": "w22-fixture-upgrade/measure.err",
      "sha256": "e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855",
      "bytes": 0,
      "reason": "Tool result stream or stderr"
    },
    {
      "path": "🧪️w13-mutate-xml-1-0-parity-2.txt",
      "sha256": "ea2829136bc4f7cde62b345de36479f611d72e396f2ecdcb12350154184f002f",
      "bytes": 86,
      "reason": "Reviewed historical command/search output"
    },
    {
      "path": "w21-four-blockers/run1-oskernel-blocked.txt",
      "sha256": "2295f089bae6b990e9e9022a659430cd7d458387bf5445d77f579c974da96b76",
      "bytes": 46457,
      "reason": "Reviewed historical command/search output"
    },
    {
      "path": "w21-four-blockers/run5.txt",
      "sha256": "4b6e55c41ff5d8b976c769760e475e38847a4a3f2c61b1444167b83f15d7cecc",
      "bytes": 771,
      "reason": "Reviewed historical command/search output"
    },
    {
      "path": "w21-four-blockers/run4.txt",
      "sha256": "ae5d683d5415d92fafcaadc53b08afad6282392a86bc4b15793dec1fb2b363b1",
      "bytes": 1992,
      "reason": "Reviewed historical command/search output"
    },
    {
      "path": "w21-four-blockers/oskernel.txt",
      "sha256": "41080cae0f2b90257cb9642a21e741f48b783303ff76b28395cc401036c41005",
      "bytes": 59,
      "reason": "Reviewed historical command/search output"
    },
    {
      "path": "w21-four-blockers/run3.txt",
      "sha256": "c50894fb91d938519d0548d8c45eb5b2360bf0531f72c85069db74ec62084336",
      "bytes": 7562,
      "reason": "Reviewed historical command/search output"
    },
    {
      "path": "w21-four-blockers/run2.txt",
      "sha256": "547d5ad04a48138169f9b52f3b932439345a5e2fd79fda48eb8e7ef9a7761fa7",
      "bytes": 29181,
      "reason": "Reviewed historical command/search output"
    },
    {
      "path": "w14-parity/mutate-tsv-iana-subject.txt",
      "sha256": "ae288ee2488376837ab665b3d4701c01b3a526fa9e467151850c0c01fe5a5bf1",
      "bytes": 84,
      "reason": "Reviewed historical command/search output"
    },
    {
      "path": "w14-parity/mutate-csv-rfc4180-parity.txt",
      "sha256": "a206c66f7008facbd75ecb76a195f97fb453924e52aee8dbe84361510adba1a6",
      "bytes": 86,
      "reason": "Reviewed historical command/search output"
    },
    {
      "path": "w14-parity/mutate-tsv-iana-parity.txt",
      "sha256": "e5a34f31908d75c9a851d1d7ed74ec9dbbac5efcb5e5256027c051e2a499a370",
      "bytes": 86,
      "reason": "Reviewed historical command/search output"
    },
    {
      "path": "w14-parity/mutate-epw-energyplus-subject.txt",
      "sha256": "163b2460f041824be6cc343c86f3ce2479ebabb59cdfd95313477d03c72e2b4c",
      "bytes": 84,
      "reason": "Reviewed historical command/search output"
    },
    {
      "path": "w14-parity/mp4-subject.txt",
      "sha256": "706972d5ec142ad270d2bb4dd8caaa321e0cf72d2345e3793e549172a97fcb77",
      "bytes": 181502,
      "reason": "Reviewed historical command/search output"
    },
    {
      "path": "w14-parity/mutate-zip-2-0-subject.txt",
      "sha256": "ae288ee2488376837ab665b3d4701c01b3a526fa9e467151850c0c01fe5a5bf1",
      "bytes": 84,
      "reason": "Reviewed historical command/search output"
    },
    {
      "path": "w14-parity/mutate-deflate-rfc1950-subject.txt",
      "sha256": "88f586d7ed9ef45108170ca259251deaadc8add67dd88185c5af913c6b9970ab",
      "bytes": 84,
      "reason": "Reviewed historical command/search output"
    },
    {
      "path": "w14-parity/mutate-epw-energyplus-parity.txt",
      "sha256": "20df0b4f14ddc014dd4a405c87a9740d42e7523f4c8751eb025fdfb72cc8a157",
      "bytes": 86,
      "reason": "Reviewed historical command/search output"
    },
    {
      "path": "w14-parity/mutate-wav-riff-pcm-subject.txt",
      "sha256": "88f586d7ed9ef45108170ca259251deaadc8add67dd88185c5af913c6b9970ab",
      "bytes": 84,
      "reason": "Reviewed historical command/search output"
    },
    {
      "path": "w14-parity/mutate-deflate-rfc1950-parity.txt",
      "sha256": "0da2f51caf69fb947eecbd700d13ebd56cc1e0b4ef0203968449c3006869f0d3",
      "bytes": 86,
      "reason": "Reviewed historical command/search output"
    },
    {
      "path": "w14-parity/contract.txt",
      "sha256": "07cb402b1ddd8c2075d15a365a6bb6e474083cfbaa31cc7aa67921637ff470da",
      "bytes": 185,
      "reason": "Reviewed historical command/search output"
    },
    {
      "path": "w14-parity/mutate-avi-1-0-subject.txt",
      "sha256": "163b2460f041824be6cc343c86f3ce2479ebabb59cdfd95313477d03c72e2b4c",
      "bytes": 84,
      "reason": "Reviewed historical command/search output"
    },
    {
      "path": "w14-parity/mutate-zip-2-0-parity.txt",
      "sha256": "e5a34f31908d75c9a851d1d7ed74ec9dbbac5efcb5e5256027c051e2a499a370",
      "bytes": 86,
      "reason": "Reviewed historical command/search output"
    },
    {
      "path": "w14-parity/mutate-csv-rfc4180-subject.txt",
      "sha256": "7631d23fe114ede9d7948f67a42d2b876075ed79275f43ce0924fbc6baf22797",
      "bytes": 84,
      "reason": "Reviewed historical command/search output"
    },
    {
      "path": "w14-parity/mutate-mp4-isobmff-parity.txt",
      "sha256": "c95b226c19759615917aeb3efa09fd3811df6f4bbb5fd49b173c2d1cabbdfd38",
      "bytes": 86,
      "reason": "Reviewed historical command/search output"
    },
    {
      "path": "w14-parity/mp4-parity.txt",
      "sha256": "c95b226c19759615917aeb3efa09fd3811df6f4bbb5fd49b173c2d1cabbdfd38",
      "bytes": 86,
      "reason": "Reviewed historical command/search output"
    },
    {
      "path": "w14-parity/mutate-mp4-isobmff-subject.txt",
      "sha256": "572c266ce0016bdbb7cd6890258417071a96d4593f59c97c7156c196516410a3",
      "bytes": 84,
      "reason": "Reviewed historical command/search output"
    },
    {
      "path": "w14-parity/mutate-binary-raw-parity.txt",
      "sha256": "14d7e96f13837b8e16edbd2e7509a4a54d554ce8d5e6d31f92719daa28529a30",
      "bytes": 84,
      "reason": "Reviewed historical command/search output"
    },
    {
      "path": "w14-parity/mutate-wav-riff-pcm-parity.txt",
      "sha256": "0da2f51caf69fb947eecbd700d13ebd56cc1e0b4ef0203968449c3006869f0d3",
      "bytes": 86,
      "reason": "Reviewed historical command/search output"
    },
    {
      "path": "w14-parity/mutate-binary-raw-subject.txt",
      "sha256": "14d7e96f13837b8e16edbd2e7509a4a54d554ce8d5e6d31f92719daa28529a30",
      "bytes": 84,
      "reason": "Reviewed historical command/search output"
    }
  ]
}
```
