def languages(protocol, environment):
    oracle, language_protocol = language_oracle()
    return oracle.notices(language_protocol, protocol["languages"], environment)
