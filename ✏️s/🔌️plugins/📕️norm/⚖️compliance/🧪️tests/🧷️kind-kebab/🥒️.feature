@capability-norm-kind-kebab @oracle-python @comparison-closed-form
Feature: Mutation kinds equal the kebab case of their variant
  The `dsl::Mutations` derive asserts at compile time that the semantic kind of a mutation leaf equals the kebab case of its variant name: consecutive capitals stay together unless followed by a lowercase letter, and digits never split. A kind that differs breaks the build of the artifact. The law is validated by an independent Python oracle over all artifacts.

  Further specification assertions, asserted by the Python oracle and not by the platform adapters:
  - Kinds follow the derive: Given a mutation leaf with variant ChangeActionVK When its kind is compared with the kebab case of the variant Then the kind is change-action-vk
  - Digits do not split: Given a mutation leaf with variant ChangeFloorPsi2 Then its kind is change-floor-psi2
  - Whole repo: Given all norm artifacts Then no mutation leaf has a kind that differs from its derived kebab case
