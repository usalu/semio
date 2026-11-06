# Ordered test signature migration

Updated ordered fixture tests, ordered update work tests, and value refusal tests for current UpdateCursor work arguments. Existing owned-entry tests already use actual remaining fuel. Work tests now compare actual work against remaining fuel rather than an outdated one-unit assumption.

No tests executed. Controlled insert tests need an actual NativeDecodeControl work loan after the production receiving API is settled; refusal error conversion requires compiler verification.