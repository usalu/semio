
    /// ⏰️ A far deadline keeps the idle sibling armed as timer keeper while a worker fires a
    /// re-arming chain: each callback wakes the sibling, which parks as keeper on the far deadline
    /// before the callback re-arms. The re-arm must reach that keeper, or the chain waits behind the
    /// far deadline at 0 % CPU — how a hub open-plan law hung for 70 min (ticket 26/09/23, `📓️wp-h6.md`).
    #[test]
    fn a_timer_re_armed_by_a_firing_callback_never_waits_behind_a_far_keeper() {
        let far = &worker_parking_fixture()["farKeeper"];
        let interval = far["intervalMs"].as_u64().unwrap();
        let bound = Duration::from_millis(far["chainBoundMs"].as_u64().unwrap());
        let pool = WorkerPool::new(WorkerPoolConfig::new(ProcessKind::HeadlessBatch, far["workers"].as_u64().unwrap() as usize));
        pool.callback_at(pool.now_ms() + far["farDeadlineMs"].as_u64().unwrap(), || {});
        fn arm(pool: WorkerPool, remaining: u64, interval: u64, done: std::sync::mpsc::SyncSender<()>) {
            let next = pool.clone();
            pool.callback_at(pool.now_ms() + interval, move || {
                if remaining == 0 {
                    done.send(()).unwrap();
                    return;
                }
                let slept = next.inner.parking.sleeps();
                next.submit(Lane::Background, Box::new(|| {}));
                let parked = Instant::now() + Duration::from_secs(5);
                while next.inner.parking.sleeps() == slept && Instant::now() < parked {
                    thread::yield_now();
                }
                arm(next, remaining - 1, interval, done);
            });
        }
        for chain in 0..far["chains"].as_u64().unwrap() {
            let (done_tx, done_rx) = std::sync::mpsc::sync_channel(1);
            arm(pool.clone(), far["rearmsPerChain"].as_u64().unwrap(), interval, done_tx);
            assert!(done_rx.recv_timeout(bound).is_ok(), "chain {chain} waited more than {bound:?} behind the far keeper");
        }
        pool.shutdown().unwrap();
    }
