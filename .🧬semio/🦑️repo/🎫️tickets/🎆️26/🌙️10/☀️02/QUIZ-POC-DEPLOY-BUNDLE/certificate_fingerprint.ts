#!/usr/bin/env bun
/** 🔏️ Prints the SHA-256 fingerprint, subject and issuer of a certificate: `served <port> <server name>` asks the TLS
 * listener on `127.0.0.1:<port>` with that SNI name, `file <pem>` reads the first certificate of a PEM bundle. */
import { X509Certificate } from "node:crypto";
import { readFileSync } from "node:fs";
import { connect } from "node:tls";

const [mode, first, second] = process.argv.slice(2);
if (mode === "file") {
  const certificate = new X509Certificate(readFileSync(first!, "utf8"));
  console.log(`${certificate.fingerprint256} names=${certificate.subjectAltName ?? ""} issuer=${(certificate.issuer ?? "").replace(/\n/gu, ",")}`);
} else {
  const socket = connect({ host: "127.0.0.1", port: Number(first), servername: second, rejectUnauthorized: false }, () => {
    const certificate = socket.getPeerCertificate();
    console.log(`${certificate.fingerprint256} names=${certificate.subjectaltname ?? ""} issuer=${JSON.stringify(certificate.issuer)}`);
    socket.end();
  });
  socket.once("error", (error) => {
    console.log(`no certificate: ${error.message}`);
    process.exit(1);
  });
}
