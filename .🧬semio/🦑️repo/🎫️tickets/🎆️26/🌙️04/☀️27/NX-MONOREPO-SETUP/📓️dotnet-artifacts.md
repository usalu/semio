# .NET Artifact Restoration

The current support library restored from Nx cache with identical file hashes and modes. Its uncached locked restore prerequisite completed before cache reuse. A separately compiled consumer referenced only the staged DLL, constructed a Step, and System.Text.Json emitted the expected wire object. No ProjectReference rebuilt the support library.

{
".nx-artifact.json": {
"hash": "7d70e41be93bea942d551f0e0b3ee74f7dc54d3123d7aca7189226cf367e96d5",
"mode": 420
},
"Semio.Repo.Test.deps.json": {
"hash": "ed1179255d56b5d5deffbd2c196057f837826613473d20e70645bd2db94198dd",
"mode": 420
},
"Semio.Repo.Test.dll": {
"hash": "e867bf57066fdb65144319210112d231993a3c0869ccdfcacdc711cce67ab416",
"mode": 420
},
"Semio.Repo.Test.pdb": {
"hash": "e8e4abd52475b63b49927acf541f31bd86731859d237eb353a0ccc85da645963",
"mode": 420
}
}
