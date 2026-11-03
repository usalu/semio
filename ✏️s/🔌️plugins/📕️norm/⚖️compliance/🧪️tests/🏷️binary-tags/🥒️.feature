@capability-norm-binary-tags @oracle-python @comparison-closed-form
Feature: Binary mutation tags are unique and agree with the protocol
  Every mutation leaf schema carries a `binaryTag` that selects the mutation kind on the binary wire. The mutations protocol file of the artifact is the only source of the wire tags; duplicate or disagreeing tags would make the codec decode wrong mutations. The law is validated by an independent Python oracle over all artifacts.

  Further specification assertions, asserted by the Python oracle and not by the platform adapters:
  - Tags are unique: Given the mutation leaf schemas of an artifact When their binary tags are collected Then no tag is used by two kinds
  - Tags agree with the protocol: Given a mutation leaf schema and the mutations protocol file When the tag of the leaf is compared with its protocol record Then both are equal And every leaf has a record
  - Protocol tags are valid: Given a mutations protocol file Then no tag is repeated And no kind is repeated
