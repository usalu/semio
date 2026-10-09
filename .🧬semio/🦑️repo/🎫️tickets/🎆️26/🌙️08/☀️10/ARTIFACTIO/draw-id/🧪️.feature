Feature: Controlled Drawing identities
 Scenario Outline: Counted canonical preimage and independent SHA-256
  Given a typed kind and ordered intrinsic octet parts
  When controlled binary IO admits the commitment
  Then preimage and digest equal the neutral witness and independent SHA-256
  And text IO publishes the canonical lowercase kind-prefixed digest
 Scenario: Semantic identity assignment
  Given an authored subtree and a complete distinct source to target key map
  When pure cloning receives the map
  Then every node and internal Boolean reference uses its admitted target key
  And missing, extra, repeated and colliding assignments refuse
