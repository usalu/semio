def languages(environment):
    oracle, protocol = language_oracle()
    return oracle.notices(protocol, policy["languages"], environment)
