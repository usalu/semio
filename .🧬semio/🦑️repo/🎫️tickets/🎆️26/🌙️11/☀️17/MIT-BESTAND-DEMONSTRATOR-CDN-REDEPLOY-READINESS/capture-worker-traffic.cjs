/** 📬️ Record original worker traffic needed to reproduce a guest command and its lifecycle. */
function captureWorkerTraffic() {
  const OriginalWorker = window.Worker;
  const encode = (_, value) => {
    if (typeof value === "bigint") return {bigint: String(value)};
    if (value instanceof Uint8Array) return {bytes: Array.from(value.subarray(0, 8192)), length: value.length};
    return value;
  };
  const record = (direction, message) => console.info("[DEBUG] authenticated-" + direction + " " + JSON.stringify(message, encode));
  window.Worker = class extends OriginalWorker {
    constructor(...args) {
      super(...args);
      this.requests = 0;
      let replies = 0;
      this.addEventListener("message", ({data}) => {
        if (replies++ < 140 && data.actorId?.startsWith("demonstrator#")) record("response", data);
      });
    }
    postMessage(message, ...args) {
      if (message.actorId?.startsWith("demonstrator#")) {
        const index = this.requests++;
        if (index < 160 || (index < 12000 && (message.events?.length || message.commandPage))) record("request", message);
      }
      return super.postMessage(message, ...args);
    }
  };
}
module.exports = {captureWorkerTraffic};
