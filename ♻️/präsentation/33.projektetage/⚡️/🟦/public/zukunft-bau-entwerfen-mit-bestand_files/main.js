"use strict";
(self.webpackChunkni_zb = self.webpackChunkni_zb || []).push([
  [792],
  {
    232: function () {
      document.addEventListener("DOMContentLoaded", function () {
        (document.querySelectorAll("[data-accordion]").forEach(function (t) {
          t.querySelectorAll("[data-accordion-trigger]").forEach(function (t) {
            t.addEventListener("click", function () {
              const e = t.closest("[data-accordion-item]").querySelector("[data-accordion-content]");
              "true" === t.getAttribute("aria-expanded")
                ? (t.setAttribute("aria-expanded", "false"),
                  e.classList.add("is-animating"),
                  e.classList.remove("is-open"),
                  setTimeout(function () {
                    (e.setAttribute("hidden", ""), e.classList.remove("is-animating"));
                  }, 300))
                : (t.setAttribute("aria-expanded", "true"),
                  e.removeAttribute("hidden"),
                  e.offsetHeight,
                  e.classList.add("is-animating"),
                  e.classList.add("is-open"),
                  setTimeout(function () {
                    e.classList.remove("is-animating");
                  }, 300));
            });
          });
        }),
          document.querySelectorAll(".tocWrapper").forEach(function (t) {
            if (window.innerWidth < 992) {
              const e = t.querySelector("[data-accordion-trigger]");
              e && e.click();
            }
          }));
      });
    },
  },
  function (t) {
    var e;
    ((e = 232), t((t.s = e)));
  },
]);
