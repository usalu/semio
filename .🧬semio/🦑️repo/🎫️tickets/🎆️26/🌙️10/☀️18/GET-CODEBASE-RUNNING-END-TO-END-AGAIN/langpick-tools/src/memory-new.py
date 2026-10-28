def languages(policy, environment):
    oracle, protocol = language_oracle()
    return oracle.notices(protocol, policy["languages"], environment)
