use super::*;
use server::throttle::{ClientKey, Throttle};
use std::collections::BTreeMap;
use std::time::Instant;

fn environment(pairs: &[(&str, &str)]) -> impl Fn(&str) -> Option<String> {
    let values: BTreeMap<String, String> = pairs.iter().map(|(name, value)| ((*name).to_string(), (*value).to_string())).collect();
    move |name| values.get(name).cloned()
}

const REQUIRED: [(&str, &str); 2] = [(DATA, "data"), (CATALOG, "catalog.json")];

#[test]
fn a_loopback_proctor_defaults_to_development() {
    let config = ProctorConfig::from_environment(environment(&REQUIRED)).expect("valid");
    assert_eq!((config.bind, config.port, config.mode, config.forwarding), (IpAddr::V4(Ipv4Addr::LOCALHOST), DEFAULT_PORT, ProctorMode::Development, Forwarding::Untrusted));
    assert_eq!(config.origins, CrossOriginPolicy::LoopbackDevelopment);
}

#[test]
fn data_and_catalog_are_required() {
    let missing = ProctorConfig::from_environment(environment(&[(CATALOG, "catalog.json")])).expect_err("data is required");
    assert!(missing.0.contains(DATA));
    let missing = ProctorConfig::from_environment(environment(&[(DATA, "data")])).expect_err("catalog is required");
    assert!(missing.0.contains(CATALOG));
}

#[test]
fn a_network_bind_needs_production_an_allowlist_and_a_proxy() {
    let bind = [(BIND, "0.0.0.0")];
    let with = |extra: &[(&str, &str)]| ProctorConfig::from_environment(environment(&[&REQUIRED[..], &bind[..], extra].concat()));
    assert!(with(&[(MODE, "development")]).expect_err("development binds loopback").0.contains("loopback"));
    assert!(with(&[]).expect_err("no allowlist").0.contains(ALLOWED_ORIGINS));
    assert!(with(&[(ALLOWED_ORIGINS, "https://quizzes.example")]).expect_err("no proxy").0.contains(TRUSTED_FORWARDING));
    let config = with(&[(ALLOWED_ORIGINS, "https://quizzes.example, https://QUIZZES.example"), (TRUSTED_FORWARDING, "proxy"), (PORT, "9000")]).expect("all three statements");
    assert_eq!((config.mode, config.port, config.forwarding), (ProctorMode::Production, 9000, Forwarding::TerminatingProxy));
    assert_eq!(config.origins, CrossOriginPolicy::Allowlist(vec!["https://quizzes.example".to_string()]));
}

#[test]
fn malformed_values_are_refused_by_name() {
    let with = |extra: (&str, &str)| ProctorConfig::from_environment(environment(&[REQUIRED[0], REQUIRED[1], extra])).expect_err("refused").0;
    assert!(with((PORT, "eighty")).contains(PORT));
    assert!(with((BIND, "localhost")).contains(BIND));
    assert!(with((MODE, "staging")).contains(MODE));
    assert!(with((TRUSTED_FORWARDING, "always")).contains(TRUSTED_FORWARDING));
    assert!(with((ALLOWED_ORIGINS, "https://example.com/path")).contains(ALLOWED_ORIGINS));
    assert!(with((ALLOWED_ORIGINS, "*")).contains(ALLOWED_ORIGINS));
}

#[test]
fn the_presence_tick_is_configurable_within_bounds() {
    let tick = |millis: Option<&str>| ProctorConfig::from_environment(environment(&[&REQUIRED[..], &millis.map(|millis| (PRESENCE_TICK_MS, millis)).into_iter().collect::<Vec<_>>()[..]].concat())).map(|config| config.presence());
    let defaults = tick(None).expect("the default tick");
    assert_eq!(defaults, PresenceSettings::default());
    assert_eq!(defaults.tick, Duration::from_millis(100));
    let fast = tick(Some("40")).expect("a faster tick");
    assert_eq!(fast, PresenceSettings { tick: Duration::from_millis(40), ..PresenceSettings::default() });
    assert_eq!(tick(Some("10")).expect("the lower bound").tick, Duration::from_millis(10));
    assert_eq!(tick(Some("1000")).expect("the upper bound").tick, Duration::from_secs(1));
    for refused in ["9", "1001", "0", "-5", "fast", "100ms"] {
        assert!(tick(Some(refused)).expect_err(refused).0.contains(PRESENCE_TICK_MS), "{refused}");
    }
}

#[test]
fn what_presence_sends_is_bounded_per_frame_and_in_total_and_both_are_configurable() {
    let with = |extra: &[(&str, &str)]| ProctorConfig::from_environment(environment(&[&REQUIRED[..], extra].concat())).map(|config| config.presence());
    let defaults = with(&[]).expect("valid");
    assert_eq!((defaults.max_frame_bytes, defaults.max_bytes_per_second), (4096, Some(16 * 1024 * 1024)));
    assert!(defaults.max_frame_bytes >= 2 * defaults.max_state_bytes, "a frame holds the largest state and another");
    let stated = with(&[(LIMIT_PRESENCE_FRAME_BYTES, "8192"), (LIMIT_PRESENCE_BYTES_PER_SECOND, "1048576")]).expect("valid");
    assert_eq!(stated, PresenceSettings { max_frame_bytes: 8192, max_bytes_per_second: Some(1_048_576), ..PresenceSettings::default() });
    for (name, refused) in [(LIMIT_PRESENCE_FRAME_BYTES, "4095"), (LIMIT_PRESENCE_FRAME_BYTES, "65537"), (LIMIT_PRESENCE_FRAME_BYTES, "4k"), (LIMIT_PRESENCE_BYTES_PER_SECOND, "65535"), (LIMIT_PRESENCE_BYTES_PER_SECOND, "4294967297"), (LIMIT_PRESENCE_BYTES_PER_SECOND, "unlimited")] {
        assert!(with(&[(name, refused)]).expect_err(refused).0.contains(name), "{name}={refused}");
    }
}

#[test]
fn origins_are_admitted_by_policy() {
    let development = CrossOriginPolicy::LoopbackDevelopment;
    assert!(development.admits("http://localhost:6061"));
    assert!(development.admits("http://127.0.0.1:8791"));
    assert!(development.admits("http://[::1]:5173"));
    assert!(!development.admits("https://quizzes.example"));
    assert!(!development.admits("http://localhost.evil.example"));
    let allowlist = CrossOriginPolicy::Allowlist(vec!["https://quizzes.example".to_string()]);
    assert!(allowlist.admits("HTTPS://quizzes.example"));
    assert!(!allowlist.admits("https://quizzes.example:444"));
    assert!(!CrossOriginPolicy::Closed.admits("http://localhost:6061"));
    assert!(is_browser_origin("https://quizzes.example:8443"));
    assert!(!is_browser_origin("https://quizzes.example:port"));
    assert!(!is_browser_origin("ftp://quizzes.example"));
}

#[test]
fn a_trusted_proxy_must_report_https() {
    assert!(Forwarding::Untrusted.secure(None));
    assert!(Forwarding::Untrusted.secure(Some("http")));
    assert!(Forwarding::TerminatingProxy.secure(Some("https")));
    assert!(Forwarding::TerminatingProxy.secure(Some("HTTPS, http")));
    assert!(!Forwarding::TerminatingProxy.secure(Some("http")));
    assert!(!Forwarding::TerminatingProxy.secure(None));
}

#[test]
fn the_edge_limits_default_to_the_production_sizing_and_every_one_is_configurable() {
    let with = |extra: &[(&str, &str)]| ProctorConfig::from_environment(environment(&[&REQUIRED[..], extra].concat()));
    let defaults = with(&[]).expect("valid");
    assert_eq!(defaults.limits, Limits { allowances: vec![Allowance { name: SIGN_UP, rate: SIGN_UPS }], ..Limits::default() }, "the framework's sizing and the proctor's own sign-up allowance");
    assert_eq!(defaults.gate().limits, defaults.limits, "the gate carries the limits to the edge");
    assert_eq!((defaults.limits.body_bytes, defaults.limits.writes, defaults.limits.reads, defaults.limits.upgrades), (Some(16_384), Rate::per_second(300, 900), Rate::per_second(600, 1800), Rate::per_second(100, 1800)));
    assert_eq!((defaults.limits.sockets_per_client, defaults.limits.sockets, defaults.limits.in_flight), (2048, 8192, 2048));
    assert_eq!((SIGN_UP, defaults.sign_ups()), ("sign-up", Rate::per_hour(100, 1200)));
    let stated = with(&[
        (LIMIT_BODY_BYTES, "8192"),
        (LIMIT_COMMANDS_PER_SECOND, "20"),
        (LIMIT_COMMANDS_BURST, "60"),
        (LIMIT_QUERIES_PER_SECOND, "10"),
        (LIMIT_QUERIES_BURST, "30"),
        (LIMIT_UPGRADES_PER_SECOND, "1"),
        (LIMIT_UPGRADES_BURST, "6"),
        (LIMIT_SIGNUPS_PER_HOUR, "30"),
        (LIMIT_SIGNUPS_BURST, "400"),
        (LIMIT_SOCKETS_PER_ADDRESS, "6"),
        (LIMIT_SOCKETS, "512"),
        (LIMIT_IN_FLIGHT, "64"),
    ])
    .expect("valid");
    let expected = Limits {
        body_bytes: Some(8192),
        writes: Rate::per_second(20, 60),
        reads: Rate::per_second(10, 30),
        upgrades: Rate::per_second(1, 6),
        allowances: vec![Allowance { name: SIGN_UP, rate: Rate::per_hour(30, 400) }],
        sockets_per_client: 6,
        sockets: 512,
        in_flight: 64,
        ..Limits::default()
    };
    assert_eq!((&stated.limits, stated.sign_ups()), (&expected, Rate::per_hour(30, 400)));
    for (name, refused) in [(LIMIT_BODY_BYTES, "1023"), (LIMIT_BODY_BYTES, "2097152"), (LIMIT_COMMANDS_PER_SECOND, "0"), (LIMIT_COMMANDS_BURST, "-1"), (LIMIT_QUERIES_PER_SECOND, "fast"), (LIMIT_SOCKETS, "1000001"), (LIMIT_IN_FLIGHT, "1e3"), (LIMIT_UPGRADES_BURST, "1.5"), (LIMIT_SIGNUPS_PER_HOUR, "0"), (LIMIT_SIGNUPS_PER_HOUR, "1000001"), (LIMIT_SIGNUPS_BURST, "many")] {
        assert!(with(&[(name, refused)]).expect_err(refused).0.contains(name), "{name}={refused}");
    }
}

#[test]
fn the_sign_up_allowance_never_refuses_a_lecture_hall_and_keeps_one_address_from_the_cap_for_weeks() {
    let start = Instant::now();
    let at = |seconds: u64| start + Duration::from_secs(seconds);
    let limits = ProctorConfig::from_environment(environment(&REQUIRED)).expect("valid").limits;
    let throttle = Throttle::new(limits, start);
    let hall = ClientKey::of(IpAddr::V4(Ipv4Addr::new(198, 51, 100, 10)));
    const WEEK: u64 = 7 * 24 * 3600;
    for week in 0..8 {
        for learner in 0..300 {
            assert_eq!(throttle.spend(hall, SIGN_UP, at(week * WEEK + learner / 5)), Ok(()), "learner {learner} of the hall signing up within a minute in week {week}");
        }
        for learner in 0..300 {
            assert_eq!(throttle.spend(hall, SIGN_UP, at(week * WEEK + 5400 + learner / 5)), Ok(()), "learner {learner} of the next hall, a lecture later, in week {week}");
        }
        for learner in 0..300 {
            assert_eq!(throttle.spend(hall, SIGN_UP, at(week * WEEK + 5460 + learner / 5)), Ok(()), "learner {learner} who cleared the browser and signs up again in week {week}");
        }
    }

    let script = ClientKey::of(IpAddr::V4(Ipv4Addr::new(203, 0, 113, 66)));
    let mut registered = 0u64;
    for second in 0..24 * 3600 {
        for _ in 0..3 {
            registered += u64::from(throttle.spend(script, SIGN_UP, at(second)).is_ok());
        }
    }
    let (burst, hourly) = (u64::from(SIGN_UPS.burst), u64::from(SIGN_UPS.tokens));
    assert!((burst + 24 * hourly - 1..=burst + 24 * hourly).contains(&registered), "three sign-ups a second for a day are cut to the burst and a day's refill: {registered}");
    let hours = (quiz::DEFAULT_LIMITS.learners - burst) / hourly;
    assert_eq!((quiz::DEFAULT_LIMITS.learners, burst, hourly, hours, hours / 24), (100_000, 1200, 100, 988, 41), "one address needs (cap − burst) / refill hours to fill the default cap: 41 days");
    assert_eq!(SIGN_UPS.refill(), Duration::from_secs(12 * 3600), "an emptied allowance is whole again within half a day");
}

#[test]
fn the_quiz_caps_default_to_the_cores_and_the_two_totals_are_configurable() {
    let with = |extra: &[(&str, &str)]| ProctorConfig::from_environment(environment(&[&REQUIRED[..], extra].concat()));
    assert_eq!(with(&[]).expect("valid").caps, quiz::DEFAULT_LIMITS);
    assert_eq!((quiz::DEFAULT_LIMITS.learners, quiz::DEFAULT_LIMITS.runs, quiz::DEFAULT_LIMITS.runs_per_quiz, quiz::DEFAULT_LIMITS.answers_per_run), (100_000, 1_000, 200, 2_000));
    assert_eq!(with(&[(MAX_LEARNERS, "300"), (MAX_RUNS, "50")]).expect("valid").caps, quiz::Limits { learners: 300, runs: 50, ..quiz::DEFAULT_LIMITS });
    for (name, refused) in [(MAX_LEARNERS, "0"), (MAX_LEARNERS, "1000001"), (MAX_LEARNERS, "many"), (MAX_RUNS, "0"), (MAX_RUNS, "-1"), (MAX_RUNS, "1e3")] {
        assert!(with(&[(name, refused)]).expect_err(refused).0.contains(name), "{name}={refused}");
    }
}
