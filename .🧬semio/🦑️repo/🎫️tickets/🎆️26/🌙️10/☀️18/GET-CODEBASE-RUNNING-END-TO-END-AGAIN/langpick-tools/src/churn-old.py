def languages(protocol, environment):
    for name in ("SEMIO_LANGUAGE", "LC_ALL", "LANG"):
        code = re.split(r"[-_.:@]", (environment.get(name) or "").lower())[0]
        if code in protocol["languages"]:
            return [code]
    return list(protocol["languages"])
