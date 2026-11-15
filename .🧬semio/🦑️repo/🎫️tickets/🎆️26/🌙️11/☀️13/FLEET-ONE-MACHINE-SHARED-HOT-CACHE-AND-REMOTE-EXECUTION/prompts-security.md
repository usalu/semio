# Owner steps for the security landing (package L)

**Update 00:40: steps 1 and 2 below are no longer needed for operation.** The cache server is now a sidecar of the group server's replica daemon (current code, port 6094, fleet interface only); nothing waits for the owner. What remains is optional cleanup and the two security items marked (still wanted).

Run on the group server (the machine that owns the NFS export and the installed cache job). No secret is printed here; nothing below needs a step on any other machine.

## 1. Replace the old write token (it was readable through the store from 22:17 and must be treated as leaked on the LAN)

The store no longer contains it (facts republished 00:10 without any secret, generation 4). The old value still sits in `/etc/semio/remote-cache.env`, and the installed old-code server still accepts it.

```
sudo sh -c 'umask 077; printf "SEMIO_REMOTE_CACHE_SERVER_READ_TOKEN=%s\nSEMIO_REMOTE_CACHE_SERVER_WRITE_TOKEN=%s\n" "$(openssl rand -base64 32 | tr -d "=+/\n")" "$(openssl rand -base64 32 | tr -d "=+/\n")" > /etc/semio/remote-cache.env'
```
(Check the variable names against the old file first: `sudo sed "s/=.*/=<hidden>/" /etc/semio/remote-cache.env`.) The new code honours these operator tokens only from the machine itself.

## 2. Restart the installed cache job so it runs the current code

It is pid 850, started 22:46, old code: it answers 401 to every signature (so every machine builds locally against it) and still binds `0.0.0.0`.

```
sudo launchctl kickstart -k system/tech.semio.fleet.cache
```
If the job passes `--host 0.0.0.0` the new code narrows it to the fleet interface by itself when it can find the network (`SEMIO_FLEET_NETWORK=192.168.178.0/24` in the job environment is the explicit way: add it to the job definition). After the restart: `repo remote-cache status` must show `authorised: true` and `tokenFrom: fleet-identity`; `repo fleet status` shows the claim table and the divergences.

## 3. Restrict the NFS export from the whole /24 to the fleet machines

Current line in `/etc/exports`: `/Users/Shared/semio-store -mapall=ueli -network 192.168.178.0 -mask 255.255.255.0`. Replace it with one line that names the machines (the eight addresses of the fleet; `<ip1> ... <ip8>`), keep the markers:

```
sudo cp /etc/exports /etc/exports.before-security
sudo sh -c 'sed -i "" "s#^/Users/Shared/semio-store .*#/Users/Shared/semio-store -mapall=ueli <ip1> <ip2> <ip3> <ip4> <ip5> <ip6> <ip7> <ip8>#" /etc/exports'
sudo nfsd checkexports && sudo nfsd update
showmount -e localhost
```
Every export client is still mapped to `ueli`, so a listed machine can write the store: that is the decided trust boundary. The generator (`🤝️shared-worktree/🗺️topology`) still renders the /24 form; changing it follows the shared-worktree landing protocol and was NOT done tonight, see `result-L.md`.

## 4. Optional, when a machine is lost or reinstalled

A reinstalled machine gets new keys; servers pin the first origin key, so it is refused until the origin is retired and re-admitted (`repo fleet` retire/admit). `repo remote-cache rotate` on a machine replaces its role keys without that step.

## 5. Optional cleanup: retire the installed system job (pid 850, old code, port 6090, binds 0.0.0.0)

```
sudo launchctl bootout system/tech.semio.fleet.cache && sudo rm -f /Library/LaunchDaemons/tech.semio.fleet.cache.plist /etc/semio/remote-cache.env
```
The job is unused now (the facts name the sidecar cache on port 6094) but still listens, and the old token in `/etc/semio/remote-cache.env` stays a leaked value until that file is gone. Step 3 (NFS export per machine) is still wanted; the generator renders it from `SEMIO_FLEET_MACHINES=ip1,ip2,...`.


## 6. Optional (package O, 2026-11-14): remove the stale cache pointer that the old provisioning wrote into the shell profiles of every machine

Not needed for operation: the resolver ignores any `SEMIO_REMOTE_CACHE_ENDPOINT` that names none of the endpoints of the healthy fleet facts (unless `SEMIO_REMOTE_CACHE_ENDPOINT_OVERRIDE` is set), and the provisioning scripts no longer write the three variables (re-running the script on a machine rewrites its block without them). To clean a machine by hand (the `SEMIO_FLEET_*` lines stay), run on each machine that already ran the old script:

```
sudo sed -i '' -e '/^export SEMIO_REMOTE_CACHE_ENDPOINT=http:\/\/192\.168\.178\.71:6090$/d' -e '/^export SEMIO_REMOTE_CACHE_ACCESS=read-write$/d' -e '/^export SEMIO_REMOTE_CACHE_INSTANCE=semio$/d' /etc/zshenv /etc/profile
```
These are exactly the lines `# >>> semio fleet` blocks of this machine hold: `/etc/zshenv` lines 2 to 4 and `/etc/profile` lines 11 to 13. A Linux machine has the same three lines in `/etc/profile.d/semio-fleet.sh`; a Windows machine has the three Machine environment variables (`[Environment]::SetEnvironmentVariable('SEMIO_REMOTE_CACHE_ENDPOINT', $null, 'Machine')`, likewise `_ACCESS` and `_INSTANCE`). Open shells keep their copy until they are restarted; the running daemons and agents never read them.
