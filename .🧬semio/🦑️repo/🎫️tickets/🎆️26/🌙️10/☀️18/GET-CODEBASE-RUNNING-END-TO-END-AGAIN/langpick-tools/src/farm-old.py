def languages(environment):
    for name in policy["languageEnvironment"]:
        value = environment.get(name)
        if value is None:
            continue
        head = re.split(r"[_.@-]", value)[0].lower()
        if head in policy["languages"]:
            return [head]
    return list(policy["languages"])
