FROM debian:trixie-slim
RUN apt-get update && apt-get install -y --no-install-recommends systemd systemd-sysv dbus procps && rm -rf /var/lib/apt/lists/*
STOPSIGNAL SIGRTMIN+3
CMD ["/sbin/init"]
