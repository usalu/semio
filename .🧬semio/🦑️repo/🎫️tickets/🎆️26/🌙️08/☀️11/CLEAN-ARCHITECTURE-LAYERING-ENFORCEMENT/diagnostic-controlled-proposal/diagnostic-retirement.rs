semio_framework_value::artifact_retire_leaf!(crate::Severity);
impl semio_framework_value::retirement::RetireOwned for crate::FaultCode {
    fn retirement(self) -> Box<dyn semio_framework_value::retirement::RetirementCursor> {
        semio_framework_value::retirement::RetireOwned::retirement(self.0)
    }
}

semio_framework_value::artifact_retire_leaf!(crate::FaultOrigin, crate::TextSpan);
semio_framework_value::artifact_retire_struct!(crate::TextError { message, span, expected });
semio_framework_value::artifact_retire_struct!(crate::ExpectedSet { tokens, keywords, keys });
semio_framework_value::artifact_retire_struct!(crate::FaultScope { plugin_id, app_id, instance_id, module, body_key });
semio_framework_value::artifact_retire_struct!(crate::FaultCause { message, code });
semio_framework_value::artifact_retire_struct!(crate::Diagnostic { code, severity, span, message, expected, scope });
semio_framework_value::artifact_retire_struct!(crate::Fault { origin, code, severity, message, scope, span, causes, retryable });
