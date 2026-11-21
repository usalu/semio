"""📮 Stage repeated retirement of the production command-origin loan."""

PROBE = "if let Some(loan)=self.loan.as_ref(){loan.validate_retirement_context(cx)?;}"
REPAIR = "if let Some(loan)=self.loan.as_ref().filter(|loan|!loan.terminal_is_empty()){loan.validate_retirement_context(cx)?;}"


def stage(source):
    if source.count(REPAIR) == 1 and PROBE not in source:
        return source
    if source.count(PROBE) != 1:
        raise ValueError("Original command-source loan validator drifted")
    return source.replace(PROBE, REPAIR)
