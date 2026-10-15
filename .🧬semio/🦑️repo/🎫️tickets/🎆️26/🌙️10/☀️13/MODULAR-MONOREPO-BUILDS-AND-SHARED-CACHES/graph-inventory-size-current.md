# Graph Inventory Observation

Read-only physical inference namespace observation, including mandatory and optional leaves; concurrent writers can change this view. This is not an authenticated quota state or retirement authority.

```json
{
  "directories": 3469,
  "files": 4050,
  "logicalBytes": 838072320,
  "elapsedMilliseconds": 1805.46,
  "largest": [
    [
      50373266,
      ".cache/b/g/m/inference/inference/p-6055/emoji-project-json.publication.json"
    ],
    [
      50373266,
      ".cache/b/g/m/inference/inference/emoji-project-json.publication.json"
    ],
    [
      49550475,
      ".cache/b/g/m/inference/inference/p-52024/emoji-project-json.publication.json"
    ],
    [
      49518688,
      ".cache/b/g/m/inference/inference/p-44203/emoji-project-json.publication.json"
    ],
    [
      49448487,
      ".cache/b/g/m/inference/inference/p-30071/emoji-project-json.publication.json"
    ],
    [
      14307436,
      ".cache/b/g/m/inference/selections/21fa376b2d32dbe51623e6a92561f609f7d8422b2d5debae541d3b2c7bdd15cf/np/29afc8937e6437cf1c6c9574.json"
    ],
    [
      14116187,
      ".cache/b/g/m/inference/selections/21fa376b2d32dbe51623e6a92561f609f7d8422b2d5debae541d3b2c7bdd15cf/np/8d998689bcb8428bf0c6d837.json"
    ],
    [
      14014284,
      ".cache/b/g/m/inference/selections/21fa376b2d32dbe51623e6a92561f609f7d8422b2d5debae541d3b2c7bdd15cf/np/f70ee23768567ce427e43b7e.json"
    ],
    [
      14014138,
      ".cache/b/g/m/inference/selections/21fa376b2d32dbe51623e6a92561f609f7d8422b2d5debae541d3b2c7bdd15cf/np/6ec9995b0ce7a0f099e1002e.json"
    ],
    [
      3785171,
      ".cache/b/g/m/inference/selections/c6f45846b4f266b93c5877578d8324a267cf44a1e2018f8a902d82b94c045596/inference/emoji-project-json.publication.json"
    ]
  ]
}
```

The measured1.8s walk makes whole-inference synchronous reconciliation per optional pack publication a concrete warm-cache cost. A future safe optimization needs a fixed per-shard usage catalog and genuine writer/reader kernel custody: dirty or crashed shards reconcile their own physical roots before publication; independent shards reserve peaks atomically in the shared bounded catalog. Ledger rows must remain fixed64/shard, not grow per selection. Compaction/deletion must update exact retained deltas in the same owned transaction, and unknown older namespace data requires authentic migration-free reconciliation under real quiescence. Clean counters alone cannot detect physical bytes left by a crashed process while another old ledger remains open. Existing full inventory remains conservative until that protocol is implemented and tested.
