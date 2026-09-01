# Txn Command Close Oracle

{
  "mode": "validate",
  "before": {
    "🦀️.rs": "15438e324d90b46ea3b9964c93a7c980bdd2825b04d7f15a989857802b6f470f",
    "🔣️.json": "e3f6b8a4c7236e52f17bbd0e637599a09730af718cad6c53d354cffebd118398",
    "🧬️schema/🔣️.json": "ad076a4cf63443b31143082f68f19baadf3ba4ca5d224dc487e9b801529d9848",
    "📜️script.ts": "49eb7b3e6eb28dee18ca55003ba00e298a55adb7d9c508511384418e08820a1d"
  },
  "after": {
    "🦀️.rs": "15438e324d90b46ea3b9964c93a7c980bdd2825b04d7f15a989857802b6f470f",
    "🔣️.json": "e3f6b8a4c7236e52f17bbd0e637599a09730af718cad6c53d354cffebd118398",
    "🧬️schema/🔣️.json": "ad076a4cf63443b31143082f68f19baadf3ba4ca5d224dc487e9b801529d9848",
    "📜️script.ts": "49eb7b3e6eb28dee18ca55003ba00e298a55adb7d9c508511384418e08820a1d"
  },
  "cases": 6,
  "results": [
    {
      "id": "before-begin-close",
      "acceptedOutputs": 1,
      "rejectsDuplicateId": true,
      "rejectsCompletionRelease": true
    },
    {
      "id": "zero-items",
      "acceptedOutputs": 1,
      "rejectsDuplicateId": true,
      "rejectsCompletionRelease": true
    },
    {
      "id": "zero-bytes",
      "acceptedOutputs": 1,
      "rejectsDuplicateId": true,
      "rejectsCompletionRelease": true
    },
    {
      "id": "short-bytes",
      "acceptedOutputs": 1,
      "rejectsDuplicateId": true,
      "rejectsCompletionRelease": true
    },
    {
      "id": "exact-external-completion",
      "acceptedOutputs": 1,
      "rejectsDuplicateId": true,
      "rejectsCompletionRelease": true
    },
    {
      "id": "exact-pending-completion",
      "acceptedOutputs": 1,
      "rejectsDuplicateId": true,
      "rejectsCompletionRelease": true
    }
  ],
  "claim": "Independent schema output selection and negative checks; native tests were not executed."
}
